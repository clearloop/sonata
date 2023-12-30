//! Manifest of the site.

use crate::{
    utils::{Prefix, Read},
    Post, Theme,
};
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

    /// The path to the favicon.
    #[serde(default = "default::favicon")]
    pub favicon: PathBuf,

    /// The output directory.
    #[serde(default = "default::out")]
    pub out: PathBuf,

    /// The path of the posts.
    #[serde(default = "default::posts")]
    pub posts: PathBuf,

    /// The path of the public directory.
    #[serde(default = "default::public")]
    pub public: PathBuf,

    /// The path of the templates.
    #[serde(default = "default::templates")]
    pub templates: PathBuf,

    /// The path of the theme.
    ///
    /// Could be a file or a directory.
    #[serde(default = "default::theme")]
    pub theme: PathBuf,
}

impl Manifest {
    /// Load manifest from the provided path.
    pub fn load(root: impl AsRef<Path>) -> Result<Self> {
        let path = root.as_ref().join("cydonia.toml");
        let manifest: Self = toml::from_str(&root.as_ref().join("cydonia.toml").read()?)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;

        Ok(manifest.abs(root))
    }

    /// Copy the public directory.
    pub fn copy_public(&self) -> Result<()> {
        if self.public.exists() {
            std::fs::copy(&self.public, self.out.join("public"))?;
        }

        Ok(())
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

    /// Get the theme.
    pub fn theme(&self) -> Result<Theme> {
        Theme::load(&self.theme)
    }

    /// Make paths absolute.
    fn abs(mut self, prefix: impl AsRef<Path>) -> Self {
        self.favicon.prefix(&prefix);
        self.out.prefix(&prefix);
        self.posts.prefix(&prefix);
        self.public.prefix(&prefix);
        self.templates.prefix(&prefix);
        self.theme.prefix(&prefix);
        self
    }
}

mod default {
    use std::{fs, path::PathBuf};

    /// Default implementation of the out directory.
    pub fn favicon() -> PathBuf {
        fs::canonicalize(PathBuf::from("favicon")).unwrap_or_default()
    }

    /// Default implementation of the out directory.
    pub fn out() -> PathBuf {
        fs::canonicalize(PathBuf::from("out")).unwrap_or_default()
    }

    /// Default implementation of the posts.
    pub fn posts() -> PathBuf {
        fs::canonicalize(PathBuf::from("posts")).unwrap_or_default()
    }

    /// Default implementation of the posts.
    pub fn public() -> PathBuf {
        fs::canonicalize(PathBuf::from("public")).unwrap_or_default()
    }

    /// Default implementation of the templates.
    pub fn templates() -> PathBuf {
        fs::canonicalize(PathBuf::from("templates")).unwrap_or_default()
    }

    /// Default implementation of the templates.
    pub fn theme() -> PathBuf {
        fs::canonicalize(PathBuf::from("theme.css")).unwrap_or_default()
    }
}
