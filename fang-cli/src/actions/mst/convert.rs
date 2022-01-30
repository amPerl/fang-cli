use clap::Parser;
use fang::{
    mst::{builder::MstBuilder, entry::Entry, Mst, MstVersionKnown},
    BinReaderExt,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

#[derive(Parser, Debug)]
pub struct ConvertOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
    /// Path to output MST
    #[clap(short = 'o', long)]
    output_path: Option<String>,
    /// New minor version
    #[clap(long)]
    minor: Option<u8>,
}

pub fn convert_mst(opts: ConvertOpts) -> anyhow::Result<()> {
    // Parse the source Mst from input_path
    let mut in_file = BufReader::new(File::open(&opts.input_path)?);
    let mst = in_file.read_le::<Mst>()?;

    // Prepare a new Mst, copying the versions and platform
    let mut mst_builder = MstBuilder::from_mst_empty(&mst)?;

    // Update the Mst version with given minor version (6, 7, 8)
    let mut new_mst_version = *mst.body.version();
    new_mst_version.set_minor(opts.minor.unwrap_or_else(|| new_mst_version.minor()));
    let new_version = MstVersionKnown::try_from(&new_mst_version)?;

    mst_builder.set_version(&new_version);

    // Add all the entries from the source Mst as references
    for entry in mst.collect_entries() {
        mst_builder.add_entry_file(
            entry.filename().to_string(),
            opts.input_path.clone(),
            entry.offset(),
            entry.size(),
            Some(entry.timestamp().timestamp() as u32),
        );
    }

    // Finalize and write the Mst with contents to specified output path or input_path.convert.mst
    let out_path = match opts.output_path {
        None => Path::new(&opts.input_path).with_extension("convert.mst"),
        Some(output_path) => Path::new(&output_path).to_path_buf(),
    };
    let mut out_file = BufWriter::new(File::create(&out_path)?);

    mst_builder.write(&mut out_file)?;

    Ok(())
}
