//! Main tests for cydonia.

use anyhow::Result;
use cydonia::{App, Post};
use std::path::PathBuf;

fn blog() -> Result<App> {
    App::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("blog"))
}

#[test]
fn app() -> Result<()> {
    blog()?;

    Ok(())
}

#[test]
fn handlebars() -> Result<()> {
    let app = blog()?;
    app.manifest.handlebars()?;

    Ok(())
}

#[test]
fn post() -> Result<()> {
    let app = blog()?;
    Post::load(app.manifest.posts.join("2023-12-29-hello-world.md"))?;

    Ok(())
}

#[test]
fn posts() -> Result<()> {
    let app = blog()?;
    app.manifest.posts()?;

    Ok(())
}
