use clap::Parser;
use fang::{BinReaderExt, BinWriterExt};
use std::{
    cell::RefCell,
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
    rc::Rc,
};

use fang::mst::Mst;

#[derive(Parser, Debug)]
pub struct ConvertOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
    /// New major version
    #[clap(long)]
    major: Option<u8>,
    /// New minor version
    #[clap(long)]
    minor: Option<u8>,
}

pub fn convert_mst(opts: ConvertOpts) -> anyhow::Result<()> {
    let mut in_file = BufReader::new(File::open(&opts.input_path)?);

    let mut mst = in_file.read_le::<Mst>()?;

    let mut content_bufs = Vec::new();
    for entry in mst.entries() {
        in_file.seek(SeekFrom::Start(entry.offset as u64))?;

        let mut buf = vec![0u8; entry.size as usize];
        in_file.read_exact(&mut buf)?;
        content_bufs.push(buf);
    }

    let out_path = Path::new(&opts.input_path).with_extension("convert.mst");
    let mut out_file = BufWriter::new(File::create(&out_path)?);

    if let Some(major) = opts.major {
        mst.body.version.set_major(major);
    }

    if let Some(minor) = opts.minor {
        mst.body.version.set_minor(minor);
    }

    let content_offset_offsets = Rc::new(RefCell::new(Vec::new()));

    out_file.write_le_args(&mst, (content_offset_offsets.clone(),))?;

    let mut content_offsets = Vec::new();
    for content_buf in content_bufs {
        let mut pos = out_file.seek(SeekFrom::Current(0))?;
        if pos % 2048 > 0 {
            let padding = 2048 - pos % 2048;
            let padding = vec![0u8; padding as usize];
            out_file.write_all(&padding)?;
            pos = out_file.seek(SeekFrom::Current(0))?;
        }

        content_offsets.push(pos);
        out_file.write_all(&content_buf)?;
    }

    for (pos, offset) in content_offset_offsets.borrow().iter().zip(content_offsets) {
        out_file.seek(SeekFrom::Start(*pos))?;

        let offset = offset as u32;
        if mst.identifier.is_little() {
            out_file.write_all(&offset.to_le_bytes())?;
        } else {
            out_file.write_all(&offset.to_be_bytes())?;
        }
    }

    Ok(())
}
