use clap::Parser;

mod mst;

#[derive(Parser)]
#[clap(about = "file type to perform action on")]
pub enum FileTypeCommand {
    #[clap(about = "Actions for MST Archives")]
    Mst {
        #[clap(subcommand)]
        cmd: mst::Command,
    },
}

impl FileTypeCommand {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            FileTypeCommand::Mst { cmd } => cmd.process(),
        }
    }
}
