use clap::Parser;

mod info;
pub use info::*;

mod list;
pub use list::*;

mod unpack;
pub use unpack::*;

/// MST subcommand to run
#[derive(Parser)]
#[clap(about)]
pub enum Command {
    /// Display metadata from the archive header
    #[clap(about)]
    Info(InfoOpts),
    /// Display the contents of the archive
    #[clap(about)]
    List(ListOpts),
    /// Unpack the resources into individual files
    #[clap(about)]
    Unpack(UnpackOpts),
}

impl Command {
    pub fn process(self) -> miette::Result<()> {
        match self {
            Command::Info(opts) => info::info_mst(opts),
            Command::List(opts) => list::list_mst(opts),
            Command::Unpack(opts) => unpack::unpack_mst(opts),
        }
    }
}
