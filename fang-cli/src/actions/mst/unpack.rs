use clap::Parser;
use fang::{
    mst::{entry::Entry, Mst},
    BinReaderExt,
};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Parser, Debug)]
pub struct UnpackOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
    /// Output directory
    #[clap(short = 'o', long)]
    output_dir: String,
}

pub fn unpack_mst(opts: UnpackOpts) -> anyhow::Result<()> {
    let mut file = BufReader::new(File::open(&opts.input_path)?);

    let mst = file.read_le::<Mst>()?;

    for entry in mst.collect_entries() {
        file.seek(SeekFrom::Start(entry.offset() as u64))?;
        let mut data = vec![0; entry.size()];
        file.read_exact(&mut data)?;

        std::fs::create_dir_all(&opts.output_dir)?;

        let mut output_file = File::create(Path::new(&opts.output_dir).join(&entry.filename()))?;

        output_file.write_all(&data)?;
    }

    Ok(())
}
