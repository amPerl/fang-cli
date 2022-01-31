use clap::Parser;
use fang::{rdg::snd_init::SndInitRdg, BinReaderExt};
use std::{fs::File, io::BufReader, path::Path};

#[derive(Parser, Debug)]
pub struct InfoOpts {
    /// Path to directory containing rdg files
    #[clap(short = 'i', long)]
    input_dir: String,
}

pub fn info_rdg(opts: InfoOpts) -> anyhow::Result<()> {
    let input_dir = Path::new(&opts.input_dir);

    let snd_init_path = input_dir.join("snd_init.rdg");
    let mut snd_init_file = BufReader::new(File::open(&snd_init_path)?);
    let snd_init_rdg = snd_init_file.read_be::<SndInitRdg>()?;
    println!("{:#?}", &snd_init_rdg);

    Ok(())
}
