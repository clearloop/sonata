//! command new

use anyhow::Result;
use ccli::{clap, clap::Parser};
use std::path::PathBuf;

/// Create a new cydonia project
#[derive(Parser, Debug)]
pub struct Init {
    /// The directory to init cydonia project
    #[clap(default_value = ".")]
    dir: PathBuf,
}

impl Init {
    /// Init project in the given directory.
    pub fn run(&self) -> Result<()> {
        println!("Init project in {:?}", self.dir);
        Ok(())
    }
}
