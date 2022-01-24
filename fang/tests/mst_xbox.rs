use std::{cell::RefCell, io::Cursor, rc::Rc};

use binrw::{BinReaderExt, BinWriterExt};
use fang::mst::Mst;

#[test]
fn rewrite_mst() {
    let in_buf = include_bytes!("./mettlearms_xb.mst");
    let mut in_file = Cursor::new(in_buf);

    let mst = in_file.read_le::<Mst>().expect("Failed to parse Mst");

    // header make sense?
    assert!(mst.identifier.is_little());
    assert_eq!(mst.entries().count(), 7);

    let mut out_buf = Vec::new();
    let mut out_file = Cursor::new(&mut out_buf);

    let content_offset_offsets = Rc::new(RefCell::new(Vec::new()));

    out_file
        .write_le_args(&mst, (content_offset_offsets.clone(),))
        .expect("Failed to write Mst");

    // yep we're definitely writing offsets
    assert_eq!(content_offset_offsets.borrow().len(), 16390); // ehh..

    let expected_content_base = mst
        .entries()
        .map(|e| e.offset)
        .min()
        .expect("gotta have entries") as usize;

    // we didn't write any content, so check if everything before that matches
    assert_eq!(out_buf.len(), expected_content_base);

    for (offset, out_byte) in out_buf.iter().enumerate() {
        assert_eq!(
            out_byte, &in_buf[offset],
            "Rewritten buffer doesn't match at offset {}",
            offset
        );
    }
}
