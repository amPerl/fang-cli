use clap::Parser;

mod info;
pub use info::*;

/// Rdg subcommand to run
#[derive(Parser)]
#[clap(about)]
pub enum Command {
    /// Parse and display Rdg contents
    #[clap(about)]
    Info(InfoOpts),
}

impl Command {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            Command::Info(opts) => info::info_rdg(opts),
        }
    }
}
