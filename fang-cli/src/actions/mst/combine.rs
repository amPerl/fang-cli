use clap::Parser;
use fang::{
    mst::{builder::MstBuilder, entry::Entry, Mst},
    BinReaderExt,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

#[derive(Parser, Debug)]
pub struct CombineOpts {
    /// Path to first MST
    #[clap(short = 'i', long)]
    input1_path: String,
    /// Path to second MST
    #[clap(short = 'j', long)]
    input2_path: String,
    /// Path to output MST
    #[clap(short = 'o', long)]
    output_path: Option<String>,
}

pub fn combine_mst(opts: CombineOpts) -> anyhow::Result<()> {
    // Parse the source Mst from input1
    let mut in_file = BufReader::new(File::open(&opts.input1_path)?);
    let mst1 = in_file.read_le::<Mst>()?;

    // Parse the source Mst from input2
    let mut in_file = BufReader::new(File::open(&opts.input2_path)?);
    let mst2 = in_file.read_le::<Mst>()?;

    // Prepare a new Mst, copying the versions and platform from input1
    let mut mst_builder = MstBuilder::from_mst_empty(&mst1)?;

    // Add all the entries from the first source Mst as references
    for entry in mst1.collect_entries() {
        mst_builder.add_entry_file(
            entry.filename().to_string(),
            opts.input1_path.clone(),
            entry.offset(),
            entry.size(),
            Some(entry.timestamp().timestamp() as u32),
        );
    }

    // Add all the entries from the second source Mst as references
    for entry in mst2.collect_entries() {
        mst_builder.add_entry_file(
            entry.filename().to_string(),
            opts.input2_path.clone(),
            entry.offset(),
            entry.size(),
            Some(entry.timestamp().timestamp() as u32),
        );
    }

    // Finalize and write the Mst with context to specified output path or input1_path.combined.mst
    let out_path = match opts.output_path {
        None => Path::new(&opts.input1_path).with_extension("combined.mst"),
        Some(output_path) => Path::new(&output_path).to_path_buf(),
    };
    let mut out_file = BufWriter::new(File::create(&out_path)?);

    mst_builder.write(&mut out_file)?;

    Ok(())
}
