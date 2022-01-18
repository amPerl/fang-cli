#![allow(dead_code)]

use binrw::{BinRead, BinWrite, WriteOptions};
use chrono::{DateTime, Utc};
use modular_bitfield::prelude::*;
use std::cell::RefCell;
use std::io::SeekFrom;
use std::rc::Rc;
#[derive(BinRead, BinWrite, Debug)]
#[bw(import(entry_offsets: Rc<RefCell<Vec<u64>>>))]
pub struct Mst {
    pub identifier: MstIdentifier,
    #[brw(is_little(identifier.is_little()))]
    #[bw(args { entry_offsets: entry_offsets })]
    pub body: MstBody,
}

impl Mst {
    pub fn entries(&self) -> impl Iterator<Item = &MstEntry> {
        self.body
            .all_entries
            .iter()
            .take(self.body.header.num_entries as usize)
    }

    pub fn support_entries(&self) -> impl Iterator<Item = &MstSupportEntry> {
        self.body
            .all_support_entries
            .iter()
            .take(self.body.header.num_support_entries as usize)
    }
}

#[derive(BinRead, BinWrite, Debug)]
#[bw(import { entry_offsets: Rc<RefCell<Vec<u64>>> })]
pub struct MstBody {
    pub version: MstVersion,
    pub header: MstHeader,

    #[br(
        args {
            count: (header.num_entries + header.num_free_entries) as usize,
            inner: binrw::args! {
                name_length: version.entry_name_length(),
                version_major: version.major(),
                version_minor: version.minor()
            }
        }
    )]
    #[bw(
        args {
            name_length: version.entry_name_length(),
            version_major: version.major(),
            version_minor: version.minor(),
            entry_offsets: entry_offsets
        }
    )]
    pub all_entries: Vec<MstEntry>,
    #[br(
        args {
            count: (header.num_support_entries + header.num_free_support_entries) as usize,
            inner: binrw::args! { name_length: version.entry_name_length() }
        }
    )]
    #[bw(
        args { name_length: version.entry_name_length() },
        align_after = 4096
    )]
    pub all_support_entries: Vec<MstSupportEntry>,
}

#[derive(BinRead, BinWrite, Debug)]
pub enum MstIdentifier {
    #[brw(magic(b"FANG"))]
    FangLittleEndian,
    #[brw(magic(b"GNAF"))]
    FangBigEndian,
    Unknown([u8; 4]),
}

impl MstIdentifier {
    pub fn is_little(&self) -> bool {
        match self {
            MstIdentifier::FangLittleEndian => true,
            MstIdentifier::FangBigEndian => false,
            MstIdentifier::Unknown(_) => true,
        }
    }
}

#[derive(BinRead, BinWrite, Debug)]
pub struct MstHeader {
    pub bytes_in_file: u32,
    pub num_entries: u32,
    pub num_free_entries: u32,
    pub num_support_entries: u32,
    pub num_free_support_entries: u32,

    pub data_offset: u32,

    pub tga_compiler_version: u32,
    pub ape_compiler_version: u32,
    pub mtx_compiler_version: u32,
    pub csv_compiler_version: u32,
    pub fnt_compiler_version: u32,
    pub sma_compiler_version: u32,
    pub gt_compiler_version: u32,
    pub wvb_compiler_version: u32,
    pub fpr_compiler_version: u32,
    pub cam_compiler_version: u32,

    _reserved: [u32; 9],
}

#[bitfield]
#[derive(BinRead, BinWrite, Clone, Debug)]
#[br(map = |x: u32| Self::from_bytes(x.to_le_bytes()))]
#[bw(map = |x: &MstVersion| Self::into_bytes(x.clone()))]
pub struct MstVersion {
    pub patch: u8,
    pub minor: u8,
    pub major: u8,
    xbox: B1,
    #[skip]
    __: B1,
    pc: B1,
    tools: B1,
    gc: B1,
    ps2: B1,
    #[skip]
    __: B2,
}

#[derive(Debug, Clone)]
pub enum MstPlatform {
    PC,
    GameCube,
    PlayStation2,
    Unknown,
}

impl MstVersion {
    pub fn platform(&self) -> MstPlatform {
        if self.pc() > 0 {
            MstPlatform::PC
        } else if self.gc() > 0 {
            MstPlatform::GameCube
        } else if self.ps2() > 0 {
            MstPlatform::PlayStation2
        } else {
            MstPlatform::Unknown
        }
    }

    pub fn entry_name_length(&self) -> usize {
        match (self.platform(), self.major(), self.minor()) {
            (MstPlatform::PlayStation2, 1, 8) => 20,
            _ => 16,
        }
    }
}

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
    #[br(count = name_length, map = super::util::vec_to_null_terminated_str)]
    #[bw(map = |x| super::util::string_to_vec(x, name_length))]
    pub filename: String,

    #[br(if(version_major >= 1 && version_minor >= 8))]
    pub flags: Option<u16>,
    #[br(if(version_major >= 1 && version_minor >= 8))]
    _reserved: Option<u16>,

    #[bw(args(entry_offsets), write_with = record_value)]
    pub offset: u32,
    pub size: u32,
    #[br(map = super::util::epoch_to_chrono)]
    #[bw(map = super::util::chrono_to_epoch)]
    pub timestamp: DateTime<Utc>,

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
    #[br(count = name_length, map = super::util::vec_to_null_terminated_str)]
    #[bw(map = |x| super::util::string_to_vec(x, name_length))]
    pub filename: String,
    #[br(map = super::util::epoch_to_chrono)]
    #[bw(map = super::util::chrono_to_epoch)]
    pub timestamp: DateTime<Utc>,
}
