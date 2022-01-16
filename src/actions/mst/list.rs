use binrw::BinReaderExt;
use clap::Parser;
use miette::IntoDiagnostic;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
};

use crate::parsers::ape::Ape;
use crate::parsers::mst::Mst;

#[derive(Parser, Debug)]
pub struct ListOpts {
    /// Path to MST
    #[clap(short = 'i', long)]
    input_path: String,
    /// Try to parse each known entry type
    #[clap(short = 'p', long)]
    try_parse: bool,
}

pub fn list_mst(opts: ListOpts) -> miette::Result<()> {
    let mut file = BufReader::new(File::open(&opts.input_path).into_diagnostic()?);

    let mst = file.read_le::<Mst>().into_diagnostic()?;

    let is_little = match mst.identifier {
        crate::parsers::mst::MstIdentifier::FangLittleEndian => true,
        crate::parsers::mst::MstIdentifier::FangBigEndian => false,
        crate::parsers::mst::MstIdentifier::Unknown(_) => true,
    };

    println!("MST Entries: ({} entries)", mst.body.header.num_entries);

    for entry in mst.entries() {
        println!(
            "{: <20}  size: {: <10}  modified: {}",
            entry.filename, entry.size, entry.timestamp
        );

        if opts.try_parse {
            let extension = entry.filename.split('.').last().unwrap();

            const KNOWN_EXTENSIONS: &[&str] = &["ape"];

            if !KNOWN_EXTENSIONS.contains(&extension) {
                continue;
            }

            let mut buffer = vec![0u8; entry.size as usize];
            file.seek(SeekFrom::Start(entry.offset as u64))
                .into_diagnostic()?;
            file.read_exact(&mut buffer).into_diagnostic()?;

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

fn try_parse_ape(cursor: &mut Cursor<Vec<u8>>, is_little: bool) -> miette::Result<()> {
    let ape = match is_little {
        true => cursor.read_le::<Ape>().into_diagnostic()?,
        false => cursor.read_be::<Ape>().into_diagnostic()?,
    };

    // eprintln!("{:?}", ape);
    dbg!(&ape);
    Ok(())
}
