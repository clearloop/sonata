//! cydonia utils.

use anyhow::Result;
use colored::Colorize;
use std::path::Path;

/// A trait for reading file with full error info.
pub trait Read {
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
