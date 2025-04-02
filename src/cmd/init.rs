//! command new

use anyhow::Result;
use ccli::{clap, clap::Parser};
use chrono::Local;
use std::{fs, path::PathBuf};

use crate::{MINIMAL_MANIFEST, TEMPLATE_POST};

/// Init command
#[derive(Parser, Debug)]
pub struct Init {
    /// The title of the site
    #[clap(short, long, default_value = "sonata")]
    title: String,
    /// The directory to init sonata project
    #[clap(default_value = ".")]
    dir: PathBuf,
}

impl Init {
    /// Init project in the given directory.
    pub fn run(&self) -> Result<()> {
        fs::create_dir_all(&self.dir)?;
        fs::create_dir_all(self.dir.join("posts"))?;
        fs::write(self.dir.join("sonata.toml"), MINIMAL_MANIFEST.trim())?;
        fs::write(
            self.dir
                .join("posts")
                .join(Local::now().format("%Y-%m-%d-hello-world.md").to_string()),
            TEMPLATE_POST.trim(),
        )?;

        Ok(())
    }
}
