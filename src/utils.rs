//! cydonia utils.

use anyhow::Result;
use colored::Colorize;
use std::path::{Path, PathBuf};

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
pub trait Prefix {
    /// Prefix self with another path.
    fn prefix(&mut self, prefix: impl AsRef<Path>);
}

impl Prefix for PathBuf {
    fn prefix(&mut self, prefix: impl AsRef<Path>) {
        if self.is_relative() {
            *self = prefix.as_ref().join(&self)
        }
    }
}
