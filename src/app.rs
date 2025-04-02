//! # App layout
//!
//! ```text
//! - blog
//!   - posts
//!     - 2023-12-29-foo-bar.md
//!   - sonata.toml
//!   - theme [ theme.css ]
//!     - index.css
//!     - post.css
//!     - theme.css
//! ```

use crate::{
    utils::{Prefix, Read},
    Manifest, Post, Templates,
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
    /// The sonata.toml manifest.
    pub manifest: Manifest,
    /// Whether to enable livereload.
    pub livereload: bool,
    /// The posts.
    pub posts: Vec<Post>,
}

impl TryFrom<Manifest> for App<'_> {
    type Error = anyhow::Error;

    fn try_from(manifest: Manifest) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_prevent_indent(true);
        handlebars.set_strict_mode(true);
        handlebars.register_embed_templates_with_extension::<Templates>(".hbs")?;

        Ok(Self {
            handlebars,
            livereload: false,
            posts: manifest.posts()?,
            manifest,
        })
    }
}

impl App<'_> {
    /// Make initial data for templates
    pub fn data(&self, mut value: Value) -> Result<Value> {
        let mut map = Map::<String, Value>::new();

        map.insert("site".into(), self.manifest.site.clone().into());
        map.insert("title".into(), self.manifest.title.clone().into());
        map.insert("base".into(), self.manifest.base.clone().into());
        map.insert("image".into(), self.manifest.image.clone().into());
        map.insert("twitter".into(), self.manifest.site.clone().into());
        map.insert(
            "favicon".into(),
            format!("/{}", self.manifest.favicon.file_name()?).into(),
        );
        map.insert(
            "description".into(),
            self.manifest.description.clone().into(),
        );

        if self.livereload {
            map.insert("livereload".into(), LIVERELOAD_ENDPOINT.into());
        }

        if let Some(data) = value.as_object_mut() {
            map.append(data);
        }

        Ok(map.into())
    }

    /// Enable livereload.
    pub fn livereload(&mut self) {
        self.livereload = true;
    }

    /// Create a new app.
    pub fn load(root: &PathBuf) -> Result<Self> {
        tracing::info!("loading app from {root:?} ...");
        Manifest::load(root)?.try_into()
    }

    /// Conditional render the site
    pub fn crender(&mut self, paths: Vec<PathBuf>) -> Result<()> {
        let mut templates_changed = false;
        for path in paths {
            if self.manifest.posts.exists() && self.manifest.posts.is_sub(&path)? {
                tracing::trace!("rendering post: {path:?} ...");
                self.render_post(Post::load(&path)?)?;
            } else if self.manifest.theme.exists() && self.manifest.theme.is_sub(&path)? {
                tracing::trace!("rendering theme: {path:?} ...");
                self.render_theme()?;
            } else if self.manifest.public.exists() && self.manifest.public.is_sub(&path)? {
                tracing::trace!("copying public: {path:?} ...");
                self.manifest.copy_public()?;
            } else if self.manifest.templates.exists() && self.manifest.templates.is_sub(&path)? {
                tracing::info!("reloading templates ...");
                templates_changed = true;
                self.register_templates()?;
            } else if self.manifest.favicon.exists() && self.manifest.favicon == path {
                tracing::trace!("skipping {path:?} ...");
            }
        }

        let posts = self.manifest.posts()?;
        if templates_changed {
            self.render_posts(posts.clone())?;
        }
        self.render_index(posts)
    }

    /// Register templates if exist.
    pub fn register_templates(&mut self) -> Result<()> {
        if self.manifest.templates.exists() {
            self.handlebars
                .register_templates_directory(&self.manifest.templates, Default::default())?;
        }

        Ok(())
    }

    /// Render the site.
    pub fn render(&mut self) -> Result<()> {
        fs::create_dir_all(&self.manifest.out)?;
        self.manifest.copy_public()?;
        self.register_templates()?;
        self.render_theme()?;

        let posts = self.manifest.posts()?;
        self.render_posts(posts.clone())?;
        self.render_index(posts)?;
        self.render_favicon()
    }

    /// Render the favicon.
    pub fn render_favicon(&self) -> Result<()> {
        if self.manifest.favicon.exists() {
            tracing::info!("rendering favicon ...");
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
                "description": post.meta.description,
                "twitter": post.meta.twitter,
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
        self.manifest.write_theme(&self.manifest.out)
    }

    /// Render a template.
    pub fn render_template(
        &self,
        name: impl AsRef<Path>,
        template: &str,
        data: Value,
    ) -> Result<()> {
        let path = self.manifest.out.join(name);
        tracing::info!("rendering {path:?} ...");
        self.handlebars
            .render_to_write(template, &self.data(data)?, File::create(path)?)
            .map_err(Into::into)
    }
}
