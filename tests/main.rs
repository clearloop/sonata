//! Main tests for cydonia.

use anyhow::Result;
use cydonia::{Manifest, Post};
use std::path::PathBuf;

fn manifest() -> Result<Manifest> {
    Manifest::load(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("blog"))
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
