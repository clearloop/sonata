//! # App layout
//!
//! ```text
//! - blog
//!   - posts
//!     - 2023-12-29-foo-bar.md
//!   - cydonia.toml
//!   - theme [ theme.css ]
//!     - index.css
//!     - post.css
//!     - theme.css
//! ```

use crate::{utils, Manifest, Post};
use anyhow::Result;
use handlebars::Handlebars;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

/// The endpoint for livereload
pub const LIVERELOAD_ENDPOINT: &str = "__livereload";

/// The root of the site.
#[derive(Clone, Debug)]
pub struct App<'app> {
    /// The handlebars instance.
    pub handlebars: Handlebars<'app>,
    /// Port for the livereload server.
    pub livereload: Option<&'static str>,
    /// The cydonia.toml manifest.
    pub manifest: Manifest,
    /// The posts.
    pub posts: Vec<Post>,
}

impl<'app> TryFrom<Manifest> for App<'app> {
    type Error = anyhow::Error;

    fn try_from(manifest: Manifest) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);

        Ok(Self {
            handlebars,
            livereload: None,
            posts: manifest.posts()?,
            manifest,
        })
    }
}

impl<'app> App<'app> {
    /// Set the port of the livereload server.
    pub fn livereload(&mut self) {
        self.livereload = Some(LIVERELOAD_ENDPOINT);
    }

    /// Create a new app.
    pub fn load(root: impl AsRef<Path>) -> Result<Self> {
        tracing::info!("loading app from {} ...", root.as_ref().display());
        Manifest::load(root)?.try_into()
    }

    /// Render the site.
    ///
    /// TODO: render specified modules.
    pub fn render(&mut self) -> Result<()> {
        tracing::info!("rendering the site to {} ...", self.manifest.out.display());
        fs::create_dir_all(&self.manifest.out)?;
        self.handlebars
            .register_templates_directory(&self.manifest.templates, Default::default())?;
        self.manifest.copy_public()?;
        self.render_css()?;

        let posts = self.manifest.posts()?;
        self.render_posts(posts.clone())?;
        self.render_index(posts)?;
        Ok(())
    }

    /// Write css to the output directory.
    pub fn render_css(&self) -> Result<()> {
        let theme = self.manifest.theme()?;
        fs::write(self.manifest.out.join("index.css"), &theme.index)?;
        fs::write(self.manifest.out.join("post.css"), &theme.post)?;

        // Copy highlight.{css, js}
        for hl in ["highlight.js", "highlight.css"] {
            let path = self.manifest.theme.join(hl);
            if path.exists() {
                fs::copy(path, self.manifest.out.join(hl))?;
            }
        }

        Ok(())
    }

    /// Render the index page.
    pub fn render_index(&self, posts: Vec<Post>) -> Result<()> {
        self.render_template(
            "index.html",
            "index",
            serde_json::json!({
                "title": self.manifest.title,
                "index": true,
                "livereload": self.livereload,
                "posts": posts,
            }),
        )
    }

    /// Render post.
    pub fn render_post(&self, post: Post) -> Result<()> {
        post.path.file_name().unwrap_or_default();
        self.render_template(
            PathBuf::from(&post.index.link),
            "post",
            serde_json::json!({
                "title": self.manifest.title,
                "livereload": self.livereload,
                "post": post,
            }),
        )
    }

    /// Render the posts.
    pub fn render_posts(&self, posts: Vec<Post>) -> Result<()> {
        fs::create_dir_all(self.manifest.out.join("posts"))?;
        for post in posts {
            self.render_post(post)?;
        }
        Ok(())
    }

    /// Render a template.
    pub fn render_template(
        &self,
        name: impl AsRef<Path>,
        template: &str,
        data: serde_json::Value,
    ) -> Result<()> {
        let path = self.manifest.out.join(name);
        tracing::debug!("rendering {path:?} ...");

        let mut file = File::create(path)?;
        let mut rendered = self.handlebars.render(template, &data)?;
        rendered = utils::fix_code_block(&rendered);
        file.write_all(rendered.as_bytes())?;
        Ok(())
    }
}
