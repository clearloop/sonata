//! cydonia utils.

use anyhow::{anyhow, Result};
use colored::Colorize;
use pulldown_cmark::{html, Options, Parser};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// A trait for reading file with full error info.
pub trait Read: Sized {
    /// Get file name with proper error info.
    fn file_name(&self) -> Result<String>;

    /// Read self to string with proper error info.
    fn read(&self) -> Result<String>;
}

impl<P> Read for P
where
    P: AsRef<Path>,
{
    fn file_name(&self) -> Result<String> {
        self.as_ref()
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Failed to get file name: {}", self.as_ref().display()))
            .map(|s| s.to_string_lossy().to_string())
    }

    fn read(&self) -> Result<String> {
        let path = self.as_ref();
        fs::read_to_string(path).map_err(|e| {
            anyhow::anyhow!(
                "Failed to read file: {}, {}",
                path.display().to_string().underline(),
                e.to_string()
            )
        })
    }
}

/// Extension trait for `PathBuf`.
pub trait Prefix: AsRef<Path> + Sized {
    /// If the target path is a sub path of self.
    fn is_sub(&self, path: impl AsRef<Path>) -> Result<bool>;

    /// Prefix self with another path.
    fn prefix(&mut self, prefix: impl AsRef<Path>);
}

impl Prefix for PathBuf {
    fn is_sub(&self, path: impl AsRef<Path>) -> Result<bool> {
        let ancestor = fs::canonicalize(self)?;
        let sub = fs::canonicalize(path)?;

        Ok(sub
            .as_os_str()
            .to_string_lossy()
            .to_string()
            .contains(ancestor.as_os_str().to_string_lossy().to_string().as_str()))
    }

    fn prefix(&mut self, prefix: impl AsRef<Path>) {
        if self.is_relative() {
            *self = prefix.as_ref().join(&self)
        }
    }
}

/// Parse markdown to html.
pub fn markdown(content: &str) -> String {
    let mut html = String::new();
    html::push_html(&mut html, Parser::new_ext(content, Options::all()));
    html
}

/// Find the directory includes `cydonia.toml`.
pub fn find_proj(base: &Path) -> Result<PathBuf> {
    if base.join("cydonia.toml").exists() {
        return Ok(base.to_path_buf());
    }

    Ok(etc::find_up("cydonia.toml")?
        .parent()
        .ok_or_else(|| anyhow!("Could not find cydonia.toml"))?
        .to_path_buf())
}
