//! The Theme for the site.

use crate::utils::Read;
use anyhow::Result;
use std::path::Path;

/// The pre-compiled highlight.js.
pub const HIGHLIGHT_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/highlight.js"));
/// The pre-compiled highlight.css.
pub const HIGHLIGHT_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/highlight.css"));
/// The default theme.
pub const DEFAULT_THEME: &str = include_str!(concat!(env!("OUT_DIR"), "/theme.css"));

/// The theme for the site.
///
/// TODO: not loading theme to memory.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Styles for the index page.
    pub index: String,
    /// Styles for the post page.
    pub post: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            index: DEFAULT_THEME.into(),
            post: DEFAULT_THEME.into(),
        }
    }
}

impl Theme {
    /// Loads theme from the given path.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        tracing::debug!("Loading theme from {path:?} ...");

        if !path.exists() {
            Ok(Default::default())
        } else if path.is_file() {
            let theme = path.read()?;

            Ok(Self {
                index: theme.clone(),
                post: theme,
            })
        } else {
            let theme = path
                .join("theme.css")
                .read()
                .unwrap_or(DEFAULT_THEME.into());
            Ok(Self {
                index: [
                    theme.clone(),
                    path.join("index.css").read().unwrap_or_default(),
                ]
                .concat(),
                post: [theme, path.join("post.css").read().unwrap_or_default()].concat(),
            })
        }
    }

    /// Writes the theme to the given path.
    pub fn write(&self, out: impl AsRef<Path>) -> Result<()> {
        let out = out.as_ref();

        std::fs::write(out.join("index.css"), &self.index)?;
        std::fs::write(out.join("post.css"), &self.post)?;
        std::fs::write(out.join("highlight.js"), HIGHLIGHT_JS)?;
        std::fs::write(out.join("highlight.css"), HIGHLIGHT_CSS)?;

        Ok(())
    }
}
