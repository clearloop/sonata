//! sonata cli
#![cfg(feature = "cli")]

pub use self::{build::Build, init::Init, serve::Serve, watch::Watch};
use ccli::{clap, clap::Parser, App};

mod build;
mod init;
mod serve;
mod watch;

/// sonata sub command.
#[derive(Debug, Parser)]
pub enum Command {
    /// Builds a sonata site from its markdown files
    Build(Box<Build>),
    /// Creates the boilerplate structure and files for a sonata site
    Init(Init),
    /// Serves a sonata site, and rebuilds it on changes
    Serve(Serve),
    /// Watches a sonata site's files and rebuilds it on changes
    Watch(Watch),
}

/// sonata command line interface
#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Sonata {
    /// The verbosity level.
    #[clap(short, long, action = clap::ArgAction::Count)]
    #[arg(global = true)]
    pub verbose: u8,

    /// The sub command.
    #[clap(subcommand)]
    pub command: Command,
}

impl App for Sonata {
    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Build(build) => build.run(),
            Command::Init(init) => init.run(),
            Command::Serve(serve) => serve.run(),
            Command::Watch(watch) => watch.run(),
        }
    }
}
