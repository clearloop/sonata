//! Command build

use crate::{App, Manifest};
use anyhow::Result;
use ccli::{clap, clap::Parser};
use colored::Colorize;
use std::path::PathBuf;

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
        let mut manifest = Manifest::load(&self.dir)?;
        manifest.merge(self.manifest.clone());

        let output = manifest.out.clone();
        App::<'_>::try_from(manifest)?.render()?;

        tracing::info!(
            "Built site at {} !",
            output.to_string_lossy().to_string().underline()
        );
        Ok(())
    }
}
