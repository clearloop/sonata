//! Manifest of the site.

use crate::{
    utils::{self, Prefix, Read},
    Post,
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
    #[cfg_attr(feature = "cli", clap(long, default_value = "Cydonia"))]
    pub title: String,

    /// The base URL of the site.
    #[serde(default = "default::base")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "/"))]
    pub base: String,

    /// The description of the site.
    #[serde(default = "Default::default")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = ""))]
    pub description: String,

    /// The path to the favicon.
    #[serde(default = "default::favicon")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "favicon.svg"))]
    pub favicon: PathBuf,

    /// The output directory.
    #[serde(default = "default::out")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value = "out"))]
    pub out: PathBuf,

    /// The path of the posts.
    #[serde(default = "default::posts")]
    #[cfg_attr(feature = "cli", clap(long, default_value = "posts"))]
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
    #[cfg_attr(feature = "cli", clap(long, default_value = "theme"))]
    pub theme: PathBuf,
}

impl Manifest {
    /// Load manifest from the provided path.
    pub fn load(root: &Path) -> Result<Self> {
        let path = utils::find_proj(root)?;
        let toml = path.join("cydonia.toml");

        tracing::info!("loading manifest from {toml:?}");
        let manifest: Self = toml::from_str(&toml.read()?)
            .map_err(|e| anyhow::anyhow!("Failed to parse {toml:?}: {e}"))?;

        Ok(manifest.abs(path))
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

    /// Write styles to the given path.
    pub fn write_theme(&self, out: &Path) -> Result<()> {
        let base = self
            .theme
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Could not find the parent path of {:?}", self.theme))?;

        for (maybe, default) in [
            ("theme.css", default::DEFAULT_THEME),
            ("highlight.css", default::HIGHLIGHT_CSS),
            ("highlight.js", default::HIGHLIGHT_JS),
        ] {
            let path = base.join(maybe);
            let hl = if path.exists() {
                path.read()?
            } else {
                default.to_string()
            };

            fs::write(out.join(maybe), hl)?;
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

impl Default for Manifest {
    fn default() -> Self {
        Self {
            title: "Cydonia".to_string(),
            base: "".to_string(),
            description: "".to_string(),
            favicon: default::favicon(),
            out: default::out(),
            posts: default::posts(),
            public: default::public(),
            templates: default::templates(),
            theme: default::theme(),
        }
    }
}

mod default {
    //! The default configurations for the manifest.
    use std::path::PathBuf;

    /// The pre-compiled highlight.js.
    pub const HIGHLIGHT_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/highlight.js"));
    /// The pre-compiled highlight.css.
    pub const HIGHLIGHT_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/highlight.css"));
    /// The default theme.
    pub const DEFAULT_THEME: &str = include_str!(concat!(env!("OUT_DIR"), "/theme.css"));

    /// Default implementation of the base URL.
    pub fn base() -> String {
        "/".to_string()
    }

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
