//! ## sonata
//!
//! The static site generator.
//!
//! For the minimal directory layout:
//!
//! ```ignore
//! .
//! ├── sonata.toml
//! └── posts
//!     └── 2024-01-01-hello-world.md
//! ```
//!
//! The full configuration:
//!
//! ```toml
//! # my-blog/sonata.toml
//! title = "sonata"         # The title of the site.
//!
//! # Default values of the optional fields.
//! # --------------------------------------
//! favicon = "favicon.svg"   # The path to the favicon.
//! out = "out"               # The path to the output directory.
//! posts = "posts"           # The path to the posts.
//! public = "public"         # The path to the public directory.
//! templates = "templates"   # The path to the templates.
//!
//! # Theme could also be a folder:
//! #
//! # - [theme]
//! #   - index.css (optional)
//! #   - post.css  (optional)
//! #   - theme.css (optional)
//! theme = "theme.css"
//! ```
//!
//! ## LICENSE
//!
//! GPL-3.0-only

mod app;
pub mod cmd;
mod manifest;
mod post;
mod utils;

pub use self::{
    app::{App, LIVERELOAD_ENDPOINT},
    manifest::{Manifest, MINIMAL_MANIFEST},
    post::{Meta, Post, TEMPLATE_POST},
};

/// The default sonata templates.
#[derive(rust_embed::RustEmbed)]
#[folder = "blog/templates"]
#[include = "*.hbs"]
pub struct Templates;

#[test]
fn embed() {
    assert!(Templates::get("post.hbs").is_some());
}
