//! Manifest of the site.

use crate::{
    utils::{Prefix, Read},
    Post, Theme,
};
use anyhow::Result;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(feature = "cli")]
use ccli::{clap, clap::Parser};

/// The minimal implementation of the manifest.
pub const MINIMAL_MANIFEST: &str = r#"
out = "out"
posts = "posts"
title = "Cydonia"
"#;

/// Manifest of the site.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", derive(Parser))]
pub struct Manifest {
    /// The name of the site.
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "Cydonia"))]
    pub title: String,

    /// The path to the favicon.
    #[serde(default = "default::favicon")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "favicon.svg"))]
    pub favicon: PathBuf,

    /// The output directory.
    #[serde(default = "Default::default")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "out"))]
    pub out: PathBuf,

    /// The path of the posts.
    #[serde(default = "default::posts")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "posts"))]
    pub posts: PathBuf,

    /// The path of the public directory.
    #[serde(default = "default::public")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "public"))]
    pub public: PathBuf,

    /// The path of the templates.
    #[serde(default = "default::templates")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "templates"))]
    pub templates: PathBuf,

    /// The path of the theme.
    ///
    /// Could be a file or a directory.
    #[serde(default = "default::theme")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "theme"))]
    pub theme: PathBuf,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            title: "Cydonia".to_string(),
            favicon: default::favicon(),
            out: default::out(),
            posts: default::posts(),
            public: default::public(),
            templates: default::templates(),
            theme: default::theme(),
        }
    }
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
        let public = self.out.join("public");
        if self.public.exists() {
            tracing::debug!(
                "copying public directory {} -> {}",
                self.public.display(),
                public.display()
            );

            etc::cp_r(&self.public, &public)?;
        }

        Ok(())
    }

    /// Merge two manifests.
    pub fn merge(&mut self, other: Manifest) {
        if other.favicon != default::favicon() {
            self.favicon = other.favicon;
        }

        if other.out != default::out() {
            self.out = other.out;
        }

        if other.posts != default::posts() {
            self.posts = other.posts;
        }

        if other.public != default::public() {
            self.public = other.public;
        }

        if other.templates != default::templates() {
            self.templates = other.templates;
        }

        if other.theme != default::theme() {
            self.theme = other.theme;
        }
    }

    /// Get all the paths.
    pub fn paths(&self) -> Vec<PathBuf> {
        vec![
            self.favicon.clone(),
            self.posts.clone(),
            self.public.clone(),
            self.templates.clone(),
            self.theme.clone(),
        ]
    }

    /// Get the posts.
    pub fn posts(&self) -> Result<Vec<Post>> {
        let mut posts = fs::read_dir(&self.posts)?
            .map(|e| Post::load(e?.path()))
            .collect::<Result<Vec<_>>>()?;

        if posts.is_empty() {
            return Ok(posts);
        }

        posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));

        let mut current_year = posts[0].meta.date.year() + 1;
        posts.iter_mut().for_each(|post| {
            let year = post.meta.date.year();
            if year < current_year {
                post.index.year = post.meta.date.format("%Y").to_string();
                current_year = year;
            }
        });

        Ok(posts)
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
    use std::path::PathBuf;

    /// Default implementation of the favicon path.
    pub fn favicon() -> PathBuf {
        PathBuf::from("favicon.svg")
    }

    /// Default implementation of the out directory.
    pub fn out() -> PathBuf {
        PathBuf::from("out")
    }

    /// Default implementation of the posts.
    pub fn posts() -> PathBuf {
        PathBuf::from("posts")
    }

    /// Default implementation of the posts.
    pub fn public() -> PathBuf {
        PathBuf::from("public")
    }

    /// Default implementation of the templates.
    pub fn templates() -> PathBuf {
        PathBuf::from("templates")
    }

    /// Default implementation of the templates.
    pub fn theme() -> PathBuf {
        PathBuf::from("theme")
    }
}

#[test]
fn minimal() {
    assert!(toml::from_str::<Manifest>(MINIMAL_MANIFEST).is_ok())
}
