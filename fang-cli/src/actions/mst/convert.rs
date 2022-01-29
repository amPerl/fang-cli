use clap::Parser;
use fang::{
    mst::{entry::Entry, Mst, MstVersionKnown},
    BinReaderExt, BinWriterExt,
};
use std::{
    cell::RefCell,
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
    rc::Rc,
};

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
    for entry in mst.collect_entries() {
        in_file.seek(SeekFrom::Start(entry.offset() as u64))?;

        let mut buf = vec![0u8; entry.size()];
        in_file.read_exact(&mut buf)?;
        content_bufs.push(buf);
    }

    let out_path = Path::new(&opts.input_path).with_extension("convert.mst");
    let mut out_file = BufWriter::new(File::create(&out_path)?);

    let mut new_mst_version = *mst.body.version();
    new_mst_version.set_major(opts.major.unwrap_or_else(|| new_mst_version.major()));
    new_mst_version.set_minor(opts.minor.unwrap_or_else(|| new_mst_version.minor()));
    let new_version = MstVersionKnown::try_from(&new_mst_version)?;

    mst.body = mst.body.convert(new_version);

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
