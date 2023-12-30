//! Command serve

use crate::cmd::Watch;
use anyhow::Result;
use ccli::{clap, clap::Parser};
use std::net::SocketAddrV4;

/// Serve command
#[derive(Parser, Debug)]
pub struct Serve {
    /// Port to listen on
    #[clap(short, long, default_value = "3000")]
    pub port: u16,

    /// Address to listen on
    #[clap(short, long, default_value = "0.0.0.0")]
    pub address: SocketAddrV4,

    /// Watch configuration
    #[clap(flatten)]
    pub watch: Watch,
}

impl Serve {
    /// Run the serve command
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
