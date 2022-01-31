use clap::Parser;

mod ape;
mod mst;
mod rdg;

#[derive(Parser)]
#[clap(about = "file type to perform action on")]
pub enum FileTypeCommand {
    #[clap(about = "Actions for MST Archives")]
    Mst {
        #[clap(subcommand)]
        cmd: mst::Command,
    },
    #[clap(about = "Actions for Ape models")]
    Ape {
        #[clap(subcommand)]
        cmd: ape::Command,
    },
    #[clap(about = "Actions for Rdg musyx files")]
    Rdg {
        #[clap(subcommand)]
        cmd: rdg::Command,
    },
}

impl FileTypeCommand {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            FileTypeCommand::Mst { cmd } => cmd.process(),
            FileTypeCommand::Ape { cmd } => cmd.process(),
            FileTypeCommand::Rdg { cmd } => cmd.process(),
        }
    }
}
