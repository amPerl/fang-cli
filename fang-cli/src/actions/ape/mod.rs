use clap::Parser;

mod info;
pub use info::*;

/// Ape subcommand to run
#[derive(Parser)]
#[clap(about)]
pub enum Command {
    /// Parse and display data in Ape
    #[clap(about)]
    Info(InfoOpts),
}

impl Command {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            Command::Info(opts) => info::info_ape(opts),
        }
    }
}
