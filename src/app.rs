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

use crate::{
    utils::{Prefix, Read},
    Manifest, Post,
};
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::{Map, Value};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

/// The endpoint for livereload
pub const LIVERELOAD_ENDPOINT: &str = "__livereload";

/// The root of the site.
#[derive(Clone, Debug)]
pub struct App<'app> {
    /// The handlebars instance.
    pub handlebars: Handlebars<'app>,
    /// The cydonia.toml manifest.
    pub manifest: Manifest,
    /// The posts.
    pub posts: Vec<Post>,
}

impl<'app> TryFrom<Manifest> for App<'app> {
    type Error = anyhow::Error;

    fn try_from(manifest: Manifest) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_prevent_indent(true);
        handlebars.set_strict_mode(true);

        Ok(Self {
            handlebars,
            posts: manifest.posts()?,
            manifest,
        })
    }
}

impl<'app> App<'app> {
    /// Make initial data for templates
    pub fn data(&self, mut value: Value) -> Result<Value> {
        let mut map = Map::<String, Value>::new();
        map.insert("title".into(), self.manifest.title.clone().into());
        if self.manifest.favicon.exists() {
            map.insert(
                "favicon".into(),
                format!("/{}", self.manifest.favicon.file_name()?).into(),
            );
        }

        if let Some(data) = value.as_object_mut() {
            map.append(data);
        }

        Ok(map.into())
    }

    /// Set the port of the livereload server.
    pub fn livereload(&mut self) -> Result<()> {
        self.handlebars
            .register_template_string("livereload", LIVERELOAD_ENDPOINT)
            .map_err(Into::into)
    }

    /// Create a new app.
    pub fn load(root: impl AsRef<Path>) -> Result<Self> {
        tracing::info!("loading app from {} ...", root.as_ref().display());
        Manifest::load(root)?.try_into()
    }

    /// Conditional render the site
    pub fn conditional_render(&mut self, paths: Vec<PathBuf>) -> Result<()> {
        for path in paths {
            if self.manifest.posts.is_sub(&path)? {
                self.render_post(Post::load(&path)?)?;
            } else if self.manifest.theme.is_sub(&path)? {
                self.render_theme()?;
            } else if self.manifest.public.is_sub(&path)? {
                self.manifest.copy_public()?;
            } else if self.manifest.favicon.is_sub(&path)? {
                self.render_favicon()?;
            } else if self.manifest.templates.is_sub(&path)? {
                self.handlebars
                    .register_templates_directory(&self.manifest.templates, Default::default())?;
            }
        }

        let posts = self.manifest.posts()?;
        self.render_index(posts)
    }

    /// Render the site.
    pub fn render(&mut self) -> Result<()> {
        fs::create_dir_all(&self.manifest.out)?;
        self.handlebars
            .register_templates_directory(&self.manifest.templates, Default::default())?;
        self.manifest.copy_public()?;
        self.render_theme()?;

        let posts = self.manifest.posts()?;
        self.render_posts(posts.clone())?;
        self.render_index(posts)
    }

    /// Render the favicon.
    pub fn render_favicon(&self) -> Result<()> {
        if self.manifest.favicon.exists() {
            let favicon = self.manifest.favicon.file_name()?;
            fs::copy(&self.manifest.favicon, self.manifest.out.join(favicon))?;
        }
        Ok(())
    }

    /// Render the index page.
    pub fn render_index(&self, posts: Vec<Post>) -> Result<()> {
        self.render_template(
            "index.html",
            "index",
            serde_json::json!({ "posts": posts, "tab": self.manifest.title }),
        )
    }

    /// Render post.
    pub fn render_post(&self, post: Post) -> Result<()> {
        self.render_template(
            PathBuf::from(&post.index.link),
            "post",
            serde_json::json!({
                "post": post,
                "tab": post.meta.title,
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

    /// Write theme to the output directory.
    pub fn render_theme(&self) -> Result<()> {
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

    /// Render a template.
    pub fn render_template(
        &self,
        name: impl AsRef<Path>,
        template: &str,
        data: Value,
    ) -> Result<()> {
        let path = self.manifest.out.join(name);
        tracing::debug!("rendering {path:?} ...");

        self.handlebars
            .render_to_write(template, &self.data(data)?, File::create(path)?)
            .map_err(Into::into)
    }
}
