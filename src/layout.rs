//! Journal layout
//!
//! - posts
//!   - 2023-12-29-foo-bar.md
//! - journal.toml
//! - global.css

use crate::post::Post;
use std::path::PathBuf;

/// The root of the site.
pub struct Layout {
    /// The root path of the site
    pub root: PathBuf,
    /// posts in the site
    pub posts: Vec<Post>,
}
