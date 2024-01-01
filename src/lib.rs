//! The static site generator.

mod app;
pub mod cmd;
mod manifest;
mod post;
mod theme;
mod utils;

pub use self::{
    app::{App, LIVERELOAD_ENDPOINT},
    manifest::Manifest,
    post::{Meta, Post},
    theme::Theme,
};

/// The default cydonia templates.
#[derive(rust_embed::RustEmbed)]
#[folder = "blog/templates"]
pub struct Templates;
