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
    fs::File,
    path::{Path, PathBuf},
};

/// The root of the site.
pub struct App<'app> {
    /// The handlebars instance.
    pub handlebars: Handlebars<'app>,
    /// The cydonia.toml manifest.
    pub manifest: Manifest,
    /// The posts.
    pub posts: Vec<Post>,
    /// The theme.
    pub theme: Theme,
}

impl<'app> App<'app> {
    /// Create a new app.
    pub fn new(root: PathBuf) -> Result<Self> {
        let manifest = Manifest::load(root)?;

        Ok(Self {
            handlebars: manifest.handlebars()?,
            posts: manifest.posts()?,
            theme: manifest.theme()?,
            manifest,
        })
    }

    /// Render a template.
    pub fn render_template(
        &self,
        name: impl AsRef<Path>,
        template: &str,
        data: serde_json::Value,
    ) -> Result<()> {
        let file = File::create(self.manifest.out.join(name.as_ref()))?;
        self.handlebars.render_to_write(template, &data, file)?;
        Ok(())
    }
}
