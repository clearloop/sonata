//! command new

use anyhow::Result;
use ccli::{clap, clap::Parser};
use std::path::PathBuf;

/// Create a new cydonia project
#[derive(Parser, Debug)]
pub struct Watch {
    /// The root of the cydonia site
    #[clap(default_value = ".")]
    pub path: PathBuf,

    /// The output directory
    #[clap(short, long, default_value = "out")]
    pub out: PathBuf,
}

impl Watch {
    /// Init project in the given directory.
    pub fn run(&self) -> Result<()> {
        println!("Watching cydonia project in {:?}", self.path);
        Ok(())
    }
}
