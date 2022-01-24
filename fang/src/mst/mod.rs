#![allow(dead_code)]

use binrw::{BinRead, BinWrite};
use modular_bitfield::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod entry;
use entry::*;

pub mod header;
use header::*;

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
            inner: (version,)
        }
    )]
    #[bw(
        args {
            version: *version,
            entry_offsets: entry_offsets
        }
    )]
    pub all_entries: Vec<MstEntry>,
    #[br(
        args {
            count: (header.num_support_entries + header.num_free_support_entries) as usize,
            inner: (version,)
        }
    )]
    #[bw(args(*version,), align_after = 2048)]
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

#[bitfield]
#[derive(BinRead, BinWrite, Copy, Clone, Debug)]
#[br(map = |x: u32| Self::from_bytes(x.to_le_bytes()))]
#[bw(map = |x: &MstVersion| Self::into_bytes(*x))]
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

impl MstVersion {
    pub fn entry_name_length(&self) -> usize {
        match (self.ps2(), self.major(), self.minor()) {
            (1, 1, 8) => 20,
            _ => 16,
        }
    }
}
