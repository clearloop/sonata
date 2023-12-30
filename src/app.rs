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

use crate::Manifest;
use anyhow::Result;
use std::path::PathBuf;

/// The root of the site.
pub struct App {
    /// The root path of the resources.
    pub root: PathBuf,
    /// The manifest of the site.
    pub manifest: Manifest,
}

impl App {
    /// Create a new app.
    pub fn new(root: PathBuf) -> Result<Self> {
        let manifest = Manifest::load(&root)?;
        Ok(Self { root, manifest })
    }
}
