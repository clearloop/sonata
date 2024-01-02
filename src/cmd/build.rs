//! Command build

use std::path::PathBuf;

use crate::{App, Manifest};
use anyhow::{anyhow, Result};
use ccli::{clap, clap::Parser};

/// Render cydonia project to the output directory.
#[derive(Debug, Parser)]
pub struct Build {
    /// The directory to build.
    #[clap(default_value = ".")]
    pub dir: PathBuf,

    /// Cydonia manifest.
    #[clap(flatten)]
    pub manifest: Manifest,
}

impl Build {
    /// Run the build command.
    pub fn run(&self) -> Result<()> {
        let proj = if self.dir == PathBuf::from(".") {
            etc::find_up("cydonia.toml")?
                .parent()
                .ok_or_else(|| anyhow!("Could not find cydonia.toml"))?
                .to_path_buf()
        } else {
            self.dir.clone()
        };

        let mut manifest = Manifest::load(proj)?;
        manifest.merge(self.manifest.clone());
        App::<'_>::try_from(manifest)?.render()
    }
}
