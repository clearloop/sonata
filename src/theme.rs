//! The Theme for the site.

use crate::utils::Read;
use anyhow::Result;
use std::path::PathBuf;

/// The theme for the site.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Styles for the index page.
    pub index: String,
    /// Styles for the post page.
    pub post: String,
}

impl Theme {
    /// Loads theme from the given path.
    pub fn load(path: PathBuf) -> Result<Self> {
        if path.is_file() {
            let theme = path.read()?;

            Ok(Self {
                index: theme.clone(),
                post: theme,
            })
        } else {
            let theme = path.join("theme.css").read().unwrap_or_default();

            let index = [theme.clone(), path.join("index.css").read()?].concat();
            let post = [theme.clone(), path.join("post.css").read()?].concat();
            Ok(Self { index, post })
        }
    }
}
