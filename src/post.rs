//! post layout.

use crate::utils::Read;
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use colored::Colorize;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

/// The template of the post.
pub const TEMPLATE_POST: &str = r#"
---
author: "cydonia"
date: "2024-01-01"
description: "This is my first post with cydonia !"
labels: ["cydonia", "rust"]
title: "Hello World!"
---

# Hello World

This is my first post with cydonia !
"#;

/// Post layout with is markdown with yaml metadata.
///
/// TODO: load posts from any directory.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Post {
    /// The content of the post in markdown.
    pub content: String,
    /// The index of the post.
    #[serde(flatten)]
    pub index: Index,
    /// The metadata of the post.
    #[serde(flatten)]
    pub meta: Meta,
    /// The path to the post.
    #[serde(skip)]
    pub path: PathBuf,
}

impl Post {
    /// Load post from path.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let mut this: Self = path.read()?.parse()?;
        this.path = path.as_ref().to_path_buf();
        this.merge_meta()
    }

    /// Merge date from the post metadata.
    pub fn merge_meta(mut self) -> Result<Self> {
        let name = self.path.with_extension("").file_name()?;
        let meta = name.splitn(4, '-').collect::<Vec<_>>();
        if meta.len() != 4 {
            return Err(anyhow::anyhow!(
                "invalid file name of post {name}, should be {}.",
                "yyyy-mm-dd-title.md".underline(),
            ));
        }

        if self.meta.date == Default::default() {
            self.meta.date = NaiveDate::from_ymd_opt(
                meta[0].parse::<i32>()?,
                meta[1].parse::<u32>()?,
                meta[2].parse::<u32>()?,
            )
            .ok_or_else(|| anyhow!("invalid date of post {name}"))?;
        }

        if self.meta.title.is_empty() {
            meta[3].split('-').for_each(|s| {
                if s.is_empty() {
                    return;
                }

                self.meta
                    .title
                    .push_str(&s[0..1].to_string().to_ascii_uppercase());
                self.meta.title.push_str(&s[1..].to_ascii_lowercase());
                self.meta.title.push(' ');
            });
        }

        Ok(self.index(name))
    }

    /// Generate the index of the post.
    pub fn index(mut self, name: String) -> Self {
        self.index.index = self.meta.date.format("%h. %d").to_string();
        self.index.link = format!("posts/{name}.html");
        self
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
        let mut content = String::new();
        html::push_html(&mut content, Parser::new_ext(markdown[2], Options::all()));

        Ok(Self {
            meta,
            content,
            ..Default::default()
        })
    }
}

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

/// The index of the post.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Index {
    /// If this post is the last post of the year.
    pub year: String,

    /// The index of the post.
    pub index: String,

    /// The link of the post.
    pub link: String,
}

#[test]
fn template() {
    assert!(Post::from_str(TEMPLATE_POST).is_ok());
}
