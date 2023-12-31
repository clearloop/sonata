//! Main tests for cydonia.

use anyhow::Result;
use cydonia::{App, Manifest, Post};
use std::path::PathBuf;

fn manifest() -> Result<Manifest> {
    Manifest::load(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("blog"))
}

#[test]
fn render() -> Result<()> {
    let mut app: App<'_> = manifest()?.try_into()?;
    app.render()?;
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
