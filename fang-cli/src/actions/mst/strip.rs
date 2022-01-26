use clap::Parser;
use fang::{BinReaderExt, BinWriterExt};
use std::{
    cell::RefCell,
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom},
    path::Path,
    rc::Rc,
};

use fang::mst::Mst;

#[derive(Parser, Debug)]
pub struct StripOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
}

pub fn strip_mst(opts: StripOpts) -> anyhow::Result<()> {
    let mut in_file = BufReader::new(File::open(&opts.input_path)?);

    let mst = in_file.read_le::<Mst>()?;

    let mut content_bufs = Vec::new();
    for entry in mst.entries() {
        in_file.seek(SeekFrom::Start(entry.offset as u64))?;

        let mut buf = vec![0u8; entry.size as usize];
        in_file.read_exact(&mut buf)?;
        content_bufs.push(buf);
    }

    let out_path = Path::new(&opts.input_path).with_extension("stripped.mst");
    let mut out_file = BufWriter::new(File::create(&out_path)?);

    let content_offset_offsets = Rc::new(RefCell::new(Vec::new()));

    out_file.write_le_args(&mst, (content_offset_offsets,))?;

    Ok(())
}
