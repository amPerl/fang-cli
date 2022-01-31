use clap::Parser;
use fang::{ape::Ape, BinReaderExt};
use std::{fs::File, io::BufReader};

#[derive(Parser, Debug)]
pub struct InfoOpts {
    /// Path to Ape
    #[clap(short = 'i', long)]
    input_path: String,
    /// Whether or not the ape is for GameCube (big endian)
    #[clap(short = 'g', long)]
    gamecube: bool,
}

pub fn info_ape(opts: InfoOpts) -> anyhow::Result<()> {
    let mut file = BufReader::new(File::open(&opts.input_path)?);

    let ape = match opts.gamecube {
        false => file.read_le::<Ape>()?,
        true => file.read_be::<Ape>()?,
    };
    println!("{:#?}", &ape);

    Ok(())
}
