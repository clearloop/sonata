//! cydonia utils.

use anyhow::Result;
use colored::Colorize;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::{
    fs,
    path::{Path, PathBuf},
};

static CODE_BLOCK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r##"((?s)<code.*?>(.*?)</code>)"##).unwrap());

/// A trait for reading file with full error info.
pub trait Read: Sized {
    /// Read self to string with proper error info.
    fn read(&self) -> Result<String>;
}

impl<P> Read for P
where
    P: AsRef<Path>,
{
    fn read(&self) -> Result<String> {
        let path = self.as_ref();
        std::fs::read_to_string(path).map_err(|e| {
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

/// Fix code block in html.
pub fn fix_code_block(html: &str) -> String {
    CODE_BLOCK_RE
        .replace_all(html, |caps: &Captures| {
            caps[1].replace(
                &caps[2],
                &caps[2].trim().replace(&format!("{:<6}", "\n"), "\n"),
            )
        })
        .into()
}
