//! Command build

use crate::{App, Manifest};
use anyhow::Result;
use ccli::{clap, clap::Parser};

/// Render cydonia project to the output directory.
#[derive(Debug, Parser)]
pub struct Build {
    #[clap(flatten)]
    manifest: Manifest,
}

impl Build {
    /// Run the build command.
    pub fn run(&self) -> Result<()> {
        let mut manifest = Manifest::load("cydonia.toml")?;
        manifest.merge(self.manifest.clone());
        App::<'_>::try_from(manifest)?.render()
    }
}
