use binrw::BinReaderExt;
use clap::Parser;
use miette::IntoDiagnostic;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::parsers::mst::Mst;

#[derive(Parser, Debug)]
pub struct UnpackOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
    /// Output directory
    #[clap(short = 'o', long)]
    output_dir: String,
}

pub fn unpack_mst(opts: UnpackOpts) -> miette::Result<()> {
    let mut file = BufReader::new(File::open(&opts.input_path).into_diagnostic()?);

    let mst = file.read_le::<Mst>().into_diagnostic()?;

    for entry in mst.entries() {
        file.seek(SeekFrom::Start(entry.offset as u64))
            .into_diagnostic()?;
        let mut data = vec![0; entry.size as usize];
        file.read_exact(&mut data).into_diagnostic()?;

        std::fs::create_dir_all(&opts.output_dir).into_diagnostic()?;

        let mut output_file =
            File::create(Path::new(&opts.output_dir).join(&entry.filename)).into_diagnostic()?;

        output_file.write_all(&data).into_diagnostic()?;
    }

    Ok(())
}
