use std::{
    cell::RefCell,
    fs::File,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
    rc::Rc,
};

use binrw::{BinReaderExt, BinWriterExt};
use fang::mst::Mst;

fn test_single(
    path: &str,
    expected_little: bool,
    expected_version: (u8, u8, u8),
    expected_xb_pc_tools_gc_ps2: (bool, bool, bool, bool, bool),
) {
    // Read in the stripped Mst
    let mut in_file = BufReader::new(File::open(path).expect("Failed to open file"));
    let mst = in_file.read_le::<Mst>().expect("Failed to parse Mst");

    // Check expected version
    assert_eq!(
        mst.identifier.is_little(),
        expected_little,
        "is little endian"
    );
    assert_eq!(
        mst.body.version().major(),
        expected_version.0,
        "major version"
    );
    assert_eq!(
        mst.body.version().minor(),
        expected_version.1,
        "minor version"
    );
    assert_eq!(
        mst.body.version().patch(),
        expected_version.2,
        "patch version"
    );
    assert_eq!(
        mst.body.version().xbox() > 0,
        expected_xb_pc_tools_gc_ps2.0,
        "is xbox"
    );
    assert_eq!(
        mst.body.version().pc() > 0,
        expected_xb_pc_tools_gc_ps2.1,
        "is pc"
    );
    assert_eq!(
        mst.body.version().tools() > 0,
        expected_xb_pc_tools_gc_ps2.2,
        "is tools"
    );
    assert_eq!(
        mst.body.version().gc() > 0,
        expected_xb_pc_tools_gc_ps2.3,
        "is gc"
    );
    assert_eq!(
        mst.body.version().ps2() > 0,
        expected_xb_pc_tools_gc_ps2.4,
        "is ps2"
    );

    // Write new stripped Mst
    let mut out_buf = Vec::new();
    let mut out_file = Cursor::new(&mut out_buf);
    out_file
        .write_le_args(&mst, (Rc::new(RefCell::new(Vec::new())),))
        .expect("Failed to write Mst");

    // Make sure the new stripped Mst matches the original as much as it was read (ignore alignment)
    let in_file_stripped_length = in_file
        .seek(SeekFrom::Current(0))
        .expect("Failed to seek file") as usize;

    // Go back to re-read input as raw bytes
    in_file
        .seek(SeekFrom::Start(0))
        .expect("Failed to seek file");
    let mut in_buf = vec![0u8; in_file_stripped_length];
    in_file
        .read_exact(&mut in_buf)
        .expect("Failed to read file");

    assert_eq!(
        &in_buf[..in_file_stripped_length],
        &out_buf[..in_file_stripped_length],
        "identical bytes"
    );
}

#[test]
fn test_heman_xb() {
    test_single(
        "../resources/mst/hm_xb_1.stripped.mst",
        true,
        (1, 6, 0),
        (true, false, true, false, false),
    );
    test_single(
        "../resources/mst/hm_xb_2.stripped.mst",
        true,
        (1, 6, 0),
        (true, false, true, false, false),
    );
}

#[test]
fn test_metalarms_gc() {
    test_single(
        "../resources/mst/ma_gc_1.stripped.mst",
        false,
        (1, 8, 0),
        (false, false, true, true, false),
    );
    test_single(
        "../resources/mst/ma_gc_2.stripped.mst",
        false,
        (1, 8, 0),
        (false, false, true, true, false),
    );
}

#[test]
fn test_metalarms_xb() {
    test_single(
        "../resources/mst/ma_xb_1.stripped.mst",
        true,
        (1, 8, 0),
        (true, false, true, false, false),
    );
    test_single(
        "../resources/mst/ma_xb_2.stripped.mst",
        true,
        (1, 8, 0),
        (true, false, true, false, false),
    );
    test_single(
        "../resources/mst/ma_xb_3.stripped.mst",
        true,
        (1, 8, 0),
        (true, false, true, false, false),
    );
    test_single(
        "../resources/mst/ma_xb_4.stripped.mst",
        true,
        (1, 7, 0),
        (true, false, true, false, false),
    );
}

#[test]
fn test_custom_xb() {
    test_single(
        "../resources/mst/unk_xb_1.stripped.mst",
        true,
        (1, 8, 0),
        (true, false, true, false, false),
    );
}
