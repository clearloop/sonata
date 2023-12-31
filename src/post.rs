//! post layout.

use crate::utils::Read;
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

/// The metadata of the post.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Meta {
    /// The author of the post.
    pub author: String,
    /// The date of the post.
    #[serde(default)]
    pub date: NaiveDate,
    /// The description of the post.
    pub description: String,
    /// The labels of the post.
    #[serde(default)]
    pub labels: Vec<String>,
    /// The title of the post.
    #[serde(default)]
    pub title: String,
}

impl FromStr for Meta {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).map_err(|e| anyhow::anyhow!(e))
    }
}

/// Post layout with is markdown with yaml metadata.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Post {
    /// The path to the post.
    #[serde(skip)]
    pub path: PathBuf,
    /// The metadata of the post.
    #[serde(flatten)]
    pub meta: Meta,
    /// The content of the post in markdown.
    pub content: String,
}

impl Post {
    /// Load post from path.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let mut this: Self = path.read()?.parse()?;
        let path = path.as_ref().to_path_buf();
        this.path = path.clone();

        let name = path
            .with_extension("")
            .file_name()
            .ok_or_else(|| anyhow!("failed to get the file name of post {path:?}"))?
            .to_string_lossy()
            .to_string();
        let meta = name.splitn(4, "-").collect::<Vec<_>>();
        if meta.len() != 4 {
            return Err(anyhow::anyhow!(
                "invalid file name of post {name}, should be {}.",
                "yyyy-mm-dd-title.md".underline(),
            ));
        }

        if this.meta.date == Default::default() {
            this.meta.date = NaiveDate::from_ymd_opt(
                meta[0].parse::<i32>()?,
                meta[1].parse::<u32>()?,
                meta[2].parse::<u32>()?,
            )
            .ok_or_else(|| anyhow!("invalid date of post {name}"))?;
        }

        if this.meta.title.is_empty() {
            meta[3].split('-').for_each(|s| {
                if s.is_empty() {
                    return;
                }

                this.meta
                    .title
                    .push_str(&s[0..1].to_string().to_ascii_uppercase());
                this.meta.title.push_str(&s[1..].to_ascii_lowercase());
                this.meta.title.push(' ');
            });
        }

        Ok(this)
    }
}

impl FromStr for Post {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let markdown = s.splitn(3, "---").collect::<Vec<_>>();
        if markdown.len() != 3 {
            return Err(anyhow::anyhow!(
                "yaml meta not found, see {} for the template.",
                "https://github.com/clearloop/cydonia"
            ));
        }

        let meta = markdown[1].parse::<Meta>()?;
        let content = markdown[2].to_string();

        Ok(Self {
            meta,
            content,
            path: Default::default(),
        })
    }
}
