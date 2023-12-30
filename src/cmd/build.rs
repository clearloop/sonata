//! Command build

use std::path::PathBuf;

use crate::{App, Manifest};
use anyhow::Result;
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
        let mut manifest = Manifest::load(self.dir.join("cydonia.toml"))?;
        manifest.merge(self.manifest.clone());
        App::<'_>::try_from(manifest)?.render()
    }
}
