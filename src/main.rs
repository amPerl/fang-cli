use clap::Parser;

mod actions;
mod parsers;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "amPerl")]
struct Opts {
    #[clap(subcommand)]
    file_type: actions::FileTypeCommand,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    opts.file_type.process()
}
