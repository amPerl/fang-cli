use clap::Parser;
use fang::{
    mst::{entry::Entry, Mst},
    BinReaderExt,
};
use std::{fs::File, io::BufReader};

#[derive(Parser, Debug)]
pub struct InfoOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
}

pub fn info_mst(opts: InfoOpts) -> anyhow::Result<()> {
    let mut file = BufReader::new(File::open(&opts.input_path)?);

    let mst = file.read_le::<Mst>()?;
    // eprintln!("{:#?}", &mst);

    println!(
        "Version: {}.{}.{} ({:?})",
        mst.body.version().major(),
        mst.body.version().minor(),
        mst.body.version().patch(),
        mst.body.version()
    );

    println!("\nEntries: {}", mst.body.header.num_entries);
    println!("Free Entries: {}", mst.body.header.num_free_entries);
    println!("Support Entries: {}", mst.body.header.num_support_entries);
    println!(
        "Free Support Entries: {}",
        mst.body.header.num_free_support_entries
    );

    let entries = mst.collect_entries();
    let earliest = entries.iter().map(|e| e.timestamp()).min();
    let latest = entries.iter().map(|e| e.timestamp()).max();

    if let Some(earliest) = earliest {
        if let Some(latest) = latest {
            println!("\nOldest entry: {}", earliest);
            println!("Newest entry: {}", latest);
        }
    }

    println!("\nCompiler versions:");
    println!(" TGA: {: >3}", mst.body.header.tga_compiler_version);
    println!(" APE: {: >3}", mst.body.header.ape_compiler_version);
    println!(" MTX: {: >3}", mst.body.header.mtx_compiler_version);
    println!(" CSV: {: >3}", mst.body.header.csv_compiler_version);
    println!(" FNT: {: >3}", mst.body.header.fnt_compiler_version);
    println!(" SMA: {: >3}", mst.body.header.sma_compiler_version);
    println!("  GT: {: >3}", mst.body.header.gt_compiler_version);
    println!(" WVB: {: >3}", mst.body.header.wvb_compiler_version);
    println!(" FPR: {: >3}", mst.body.header.fpr_compiler_version);
    println!(" CAM: {: >3}", mst.body.header.cam_compiler_version);

    Ok(())
}
