//! Command serve

use anyhow::Result;
use ccli::{clap, clap::Parser};
use std::{net::SocketAddrV4, path::PathBuf};

#[derive(Parser, Debug)]
pub struct Serve {
    /// Port to listen on
    pub port: u16,

    /// Address to listen on
    pub address: SocketAddrV4,

    /// Path to the output directory
    #[clap(short, long, default_value = "out")]
    pub out: PathBuf,
}

impl Serve {
    /// Run the serve command
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
