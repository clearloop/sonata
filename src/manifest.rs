//! Manifest of the site.

use crate::{utils::Read, Post};
use anyhow::Result;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Manifest of the site.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manifest {
    /// The name of the site.
    pub name: String,

    /// The path of the posts.
    #[serde(default = "Manifest::default_posts")]
    pub posts: PathBuf,

    /// The path of the templates.
    #[serde(default = "Manifest::default_templates")]
    pub templates: PathBuf,

    /// The path of the theme.
    ///
    /// Could be a file or a directory.
    #[serde(default = "Manifest::default_theme")]
    pub theme: PathBuf,
}

impl Manifest {
    /// Load manifest from the provided path.
    pub fn load(root: impl AsRef<Path>) -> Result<Self> {
        let path = root.as_ref().join("cydonia.toml");
        let mut manifest: Self = toml::from_str(&root.as_ref().join("cydonia.toml").read()?)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;

        if manifest.posts.is_relative() {
            manifest.posts = root.as_ref().join(&manifest.posts);
        }

        if manifest.templates.is_relative() {
            manifest.templates = root.as_ref().join(&manifest.templates);
        }

        Ok(manifest)
    }

    /// Get the posts.
    pub fn posts(&self) -> Result<Vec<Post>> {
        fs::read_dir(&self.posts)?
            .map(|e| Post::load(e?.path()))
            .collect()
    }

    /// Resource the templates into handlebars.
    pub fn handlebars<'r>(&self) -> Result<Handlebars<'r>> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_templates_directory(".hbs", &self.templates)?;

        Ok(handlebars)
    }

    /// Default implementation of the posts.
    pub fn default_posts() -> PathBuf {
        fs::canonicalize(PathBuf::from("posts")).unwrap_or_default()
    }

    /// Default implementation of the templates.
    pub fn default_templates() -> PathBuf {
        fs::canonicalize(PathBuf::from("templates")).unwrap_or_default()
    }

    /// Default implementation of the templates.
    pub fn default_theme() -> PathBuf {
        fs::canonicalize(PathBuf::from("theme.css")).unwrap_or_default()
    }
}
