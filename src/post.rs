//! post layout.

use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::Path, str::FromStr};

/// The metadata of the post.
#[derive(Clone, Debug, Deserialize)]
pub struct Meta {
    /// The author of the post.
    pub author: String,
    /// The date of the post.
    pub date: Option<String>,
    /// The description of the post.
    pub description: String,
    /// The labels of the post.
    pub labels: Vec<String>,
    /// The title of the post.
    pub title: String,
}

impl FromStr for Meta {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).map_err(|e| anyhow::anyhow!(e))
    }
}

/// Post layout with is markdown with yaml metadata.
#[derive(Clone, Debug)]
pub struct Post {
    /// The metadata of the post.
    pub meta: Meta,
    /// The content of the post in markdown.
    pub content: String,
}

impl Post {
    /// Load post from path.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        fs::read_to_string(path.as_ref())
            .map_err(|e| {
                anyhow::anyhow!(
                    "failed to read post from {}: {}",
                    path.as_ref().display(),
                    e
                )
            })?
            .parse()
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

        Ok(Self { meta, content })
    }
}
