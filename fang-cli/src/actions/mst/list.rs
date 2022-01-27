use clap::Parser;
use fang::{
    mst::{
        entry::{Entry, SupportEntry},
        Mst,
    },
    BinReaderExt,
};
use std::{fs::File, io::BufReader};

#[derive(Parser, Debug)]
pub struct ListOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
}

pub fn list_mst(opts: ListOpts) -> anyhow::Result<()> {
    let mut file = BufReader::new(File::open(&opts.input_path)?);

    let mst = file.read_le::<Mst>()?;

    println!("MST Entries: ({} entries)", mst.body.header.num_entries);

    for entry in mst.collect_entries() {
        println!(
            "{: <20}  pos: {: <10}  size: {: <10}  modified: {}",
            entry.filename(),
            entry.offset(),
            entry.size(),
            entry.timestamp()
        );
    }

    println!(
        "MST Support Entries: ({} entries)",
        mst.body.header.num_support_entries
    );
    for support_entry in mst.collect_support_entries() {
        println!(
            "{: <20}  modified: {}",
            support_entry.filename(),
            support_entry.timestamp()
        );
    }

    Ok(())
}
