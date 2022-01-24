use clap::Parser;
use fang::BinReaderExt;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
};

use fang::ape::Ape;
use fang::mst::Mst;

#[derive(Parser, Debug)]
pub struct ListOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
    /// Try to parse each known entry type
    #[clap(short = 'p', long)]
    try_parse: bool,
}

pub fn list_mst(opts: ListOpts) -> anyhow::Result<()> {
    let mut file = BufReader::new(File::open(&opts.input_path)?);

    let mst = file.read_le::<Mst>()?;

    let is_little = match mst.identifier {
        fang::mst::MstIdentifier::FangLittleEndian => true,
        fang::mst::MstIdentifier::FangBigEndian => false,
        fang::mst::MstIdentifier::Unknown(_) => true,
    };

    println!("MST Entries: ({} entries)", mst.body.header.num_entries);

    for entry in mst.entries() {
        println!(
            "{: <20}  pos: {: <10}  size: {: <10}  modified: {}",
            entry.filename, entry.offset, entry.size, entry.timestamp
        );

        if opts.try_parse {
            let extension = entry.filename.split('.').last().unwrap();

            const KNOWN_EXTENSIONS: &[&str] = &["ape"];

            if !KNOWN_EXTENSIONS.contains(&extension) {
                continue;
            }

            let mut buffer = vec![0u8; entry.size as usize];
            file.seek(SeekFrom::Start(entry.offset as u64))?;
            file.read_exact(&mut buffer)?;

            let mut buffer_cursor = Cursor::new(buffer);

            if extension == "ape" {
                try_parse_ape(&mut buffer_cursor, is_little)?
            }
        }
    }

    println!(
        "MST Support Entries: ({} entries)",
        mst.body.header.num_support_entries
    );
    for support_entry in mst.support_entries() {
        println!(
            "{: <20}  modified: {}",
            support_entry.filename, support_entry.timestamp
        );
    }

    Ok(())
}

fn try_parse_ape(cursor: &mut Cursor<Vec<u8>>, is_little: bool) -> anyhow::Result<()> {
    let ape = match is_little {
        true => cursor.read_le::<Ape>()?,
        false => cursor.read_be::<Ape>()?,
    };

    // eprintln!("{:?}", ape);
    if ape.light_count > 0 {
        dbg!(&ape);
        anyhow::bail!("hello");
    }

    Ok(())
}
