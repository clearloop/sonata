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

use crate::{Manifest, Post, Theme};
use anyhow::Result;
use handlebars::Handlebars;
use std::{
    fs::{self, File},
    path::Path,
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
    /// The theme.
    pub theme: Theme,
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
            theme: manifest.theme()?,
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
            .register_templates_directory(".hbs", &self.manifest.templates)?;
        self.manifest.copy_public()?;
        self.render_css()?;
        self.render_index()?;
        Ok(())
    }

    /// Write css to the output directory.
    pub fn render_css(&self) -> Result<()> {
        tracing::debug!("rendering css ...");
        fs::write(self.manifest.out.join("index.css"), &self.theme.index)?;
        fs::write(self.manifest.out.join("post.css"), &self.theme.post).map_err(Into::into)
    }

    /// Render the index page.
    pub fn render_index(&self) -> Result<()> {
        self.render_template(
            "index.html",
            "index",
            serde_json::json!({
                "title": self.manifest.title,
                "index": true,
                "livereload": self.livereload,
            }),
        )
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

        let file = File::create(path)?;
        self.handlebars.render_to_write(template, &data, file)?;
        Ok(())
    }
}
