use clap::Parser;

mod info;
pub use info::*;

mod list;
pub use list::*;

mod unpack;
pub use unpack::*;

mod convert;
pub use convert::*;

mod combine;
pub use combine::*;

mod strip;
pub use strip::*;

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
    /// Read the file and write it back as a different version
    #[clap(about)]
    Convert(ConvertOpts),
    /// Combine two files and write them into a new file
    #[clap(about)]
    Combine(CombineOpts),
    /// Read the file and write it back without the content
    #[clap(about)]
    Strip(StripOpts),
}

impl Command {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            Command::Info(opts) => info::info_mst(opts),
            Command::List(opts) => list::list_mst(opts),
            Command::Unpack(opts) => unpack::unpack_mst(opts),
            Command::Convert(opts) => convert::convert_mst(opts),
            Command::Combine(opts) => combine::combine_mst(opts),
            Command::Strip(opts) => strip::strip_mst(opts),
        }
    }
}
