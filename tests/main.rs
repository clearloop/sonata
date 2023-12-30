//! Main tests for cydonia.

use anyhow::Result;
use cydonia::{App, Manifest, Post};
use std::path::PathBuf;

fn app() -> Result<App> {
    App::load(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("blog"))
}

fn manifest() -> Result<Manifest> {
    Manifest::load(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("blog")
            .join("cydonia.toml"),
    )
}

#[test]
fn resource() -> Result<()> {
    app()?;
    Ok(())
}

#[test]
fn handlebars() -> Result<()> {
    manifest()?.handlebars()?;
    Ok(())
}

#[test]
fn post() -> Result<()> {
    Post::load(manifest()?.posts.join("2023-12-29-hello-world.md"))?;
    Ok(())
}

#[test]
fn posts() -> Result<()> {
    manifest()?.posts()?;
    Ok(())
}
