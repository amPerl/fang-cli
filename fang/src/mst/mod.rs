#![allow(dead_code)]

use binrw::{BinRead, BinWrite};
use modular_bitfield::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod entry;
use entry::*;

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
        align_after = 2048
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
