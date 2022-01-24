use binrw::{BinRead, BinWrite, WriteOptions};
use chrono::{DateTime, Utc};
use std::cell::RefCell;
use std::io::SeekFrom;
use std::rc::Rc;

#[derive(BinRead, BinWrite, Debug)]
#[br(import {
    name_length: usize,
    version_major: u8,
    version_minor: u8
})]
#[bw(import {
    name_length: usize,
    version_major: u8,
    version_minor: u8,
    entry_offsets: Rc<RefCell<Vec<u64>>>
})]
pub struct MstEntry {
    #[br(count = name_length, map = crate::util::vec_to_null_terminated_str)]
    #[bw(map = |x| crate::util::string_to_vec(x, name_length))]
    pub filename: String,

    #[bw(map(|x| if version_major >= 1 && version_minor >= 8 { x.or(Some(0)) } else { None }))]
    #[br(if(version_major >= 1 && version_minor >= 8))]
    pub flags: Option<u16>,

    #[bw(map(|x| if version_major >= 1 && version_minor >= 8 { x.or(Some(0)) } else { None }))]
    #[br(if(version_major >= 1 && version_minor >= 8))]
    _reserved: Option<u16>,

    #[bw(args(entry_offsets), write_with = record_value)]
    pub offset: u32,
    pub size: u32,
    #[br(map = crate::util::epoch_to_chrono)]
    #[bw(map = crate::util::chrono_to_epoch)]
    pub timestamp: DateTime<Utc>,

    #[bw(map(|x| if version_major >= 1 && version_minor >= 7 { x.or(Some(0)) } else { None }))]
    #[br(if(version_major >= 1 && version_minor >= 7))]
    pub crc: Option<u32>,
}

fn record_value<W: binrw::io::Write + binrw::io::Seek>(
    &value: &u32,
    writer: &mut W,
    opts: &WriteOptions,
    args: (Rc<RefCell<Vec<u64>>>,),
) -> binrw::BinResult<()> {
    let pos = writer.seek(SeekFrom::Current(0))?;
    args.0.borrow_mut().push(pos);
    value.write_options(writer, opts, ())
}

#[derive(BinRead, BinWrite, Debug)]
#[brw(import { name_length: usize })]
pub struct MstSupportEntry {
    #[br(count = name_length, map = crate::util::vec_to_null_terminated_str)]
    #[bw(map = |x| crate::util::string_to_vec(x, name_length))]
    pub filename: String,
    #[br(map = crate::util::epoch_to_chrono)]
    #[bw(map = crate::util::chrono_to_epoch)]
    pub timestamp: DateTime<Utc>,
}