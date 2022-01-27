#![allow(dead_code)]

use binrw::{BinRead, BinWrite};
use modular_bitfield::prelude::*;

pub mod entry;
use entry::*;

pub mod header;
use header::*;

#[derive(BinRead, BinWrite, Debug)]
#[bw(import(entry_offsets: EntryOffsets))]
pub struct Mst {
    pub identifier: MstIdentifier,
    #[brw(is_little(identifier.is_little()))]
    #[bw(args(entry_offsets))]
    pub body: MstBody,
}

impl Mst {
    pub fn collect_entries(&self) -> Vec<CanonicalEntry> {
        match &self.body.entries {
            Entries::V180PS2 { inner } => inner.entries.to_vec(),
            Entries::V180 { inner } => inner.entries.iter().map(|e| e.clone().into()).collect(),
            Entries::V170 { inner } => inner.entries.iter().map(|e| e.clone().into()).collect(),
            Entries::V160 { inner } => inner.entries.iter().map(|e| e.clone().into()).collect(),
        }
    }

    pub fn collect_support_entries(&self) -> Vec<CanonicalSupportEntry> {
        match &self.body.entries {
            Entries::V180PS2 { inner } => inner.support_entries.to_vec(),
            Entries::V180 { inner } => inner
                .support_entries
                .iter()
                .map(|e| e.clone().into())
                .collect(),
            Entries::V170 { inner } => inner
                .support_entries
                .iter()
                .map(|e| e.clone().into())
                .collect(),
            Entries::V160 { inner } => inner
                .support_entries
                .iter()
                .map(|e| e.clone().into())
                .collect(),
        }
    }
}

#[derive(BinRead, BinWrite, Debug)]
#[bw(import(entry_offsets: EntryOffsets))]
pub struct MstBody {
    version: MstVersion,
    pub header: MstHeader,
    #[br(args(version, header))]
    #[bw(args(*version, entry_offsets))]
    pub entries: Entries,
}

impl MstBody {
    pub fn version(&self) -> &MstVersion {
        &self.version
    }

    pub fn convert(&mut self, major: u8, minor: u8, patch: u8) -> anyhow::Result<()> {
        match (major, minor, patch) {
            (1, 6 | 7 | 8, 0) => {
                self.version.set_minor(minor);
                self.entries = self
                    .entries
                    .convert(major, minor, patch, self.version.ps2() > 0)?;
            }
            _ => anyhow::bail!("Unknown target version"),
        }

        Ok(())
    }
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
#[bw(map = |x: &MstVersion| u32::from_le_bytes(Self::into_bytes(*x)))]
pub struct MstVersion {
    pub patch: u8,
    pub minor: u8,
    pub major: u8,
    pub xbox: B1,
    #[skip]
    __: B1,
    pub pc: B1,
    pub tools: B1,
    pub gc: B1,
    pub ps2: B1,
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
