#![allow(dead_code)]

use binrw::{BinRead, BinWrite};
use modular_bitfield::prelude::*;

pub mod entry;
use entry::{CanonicalEntry, CanonicalSupportEntry, EntryOffsets};

pub mod entries;
use entries::Entries;

pub mod header;
use header::MstHeader;

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

#[derive(BinRead, BinWrite, Debug, Clone)]
#[bw(import(entry_offsets: EntryOffsets))]
pub struct MstBody {
    #[br(assert(MstVersionKnown::try_from(&version).is_ok(), "not a known mst version: {:?}", version))]
    #[bw(assert(MstVersionKnown::try_from(version).is_ok(), "not a known mst version: {:?}", version))]
    version: MstVersion,

    pub header: MstHeader,

    #[br(args(MstVersionKnown::try_from(&version).unwrap(), header))]
    #[bw(args(MstVersionKnown::try_from(version).unwrap(), entry_offsets))]
    pub entries: Entries,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MstVersionKnown {
    V180PS2,
    V180,
    V170,
    V160,
}

impl TryFrom<&MstVersion> for MstVersionKnown {
    type Error = anyhow::Error;

    fn try_from(mst_version: &MstVersion) -> anyhow::Result<Self> {
        match (
            mst_version.major(),
            mst_version.minor(),
            mst_version.patch(),
            mst_version.ps2() > 0,
        ) {
            (1, 8, 0, true) => Ok(MstVersionKnown::V180PS2),
            (1, 8, 0, false) => Ok(MstVersionKnown::V180),
            (1, 7, 0, _) => Ok(MstVersionKnown::V170),
            (1, 6, 0, _) => Ok(MstVersionKnown::V160),
            _ => anyhow::bail!(
                "{}.{}.{} is not a known Mst version",
                mst_version.major(),
                mst_version.minor(),
                mst_version.patch()
            ),
        }
    }
}

impl MstBody {
    pub fn version(&self) -> &MstVersion {
        &self.version
    }

    pub fn convert(&self, new_version: MstVersionKnown) -> Self {
        if MstVersionKnown::try_from(self.version()).unwrap() == new_version {
            return (*self).clone();
        }

        let mut new_mst_version = self.version;

        match new_version {
            MstVersionKnown::V180PS2 => new_mst_version.set_minor(8),
            MstVersionKnown::V180 => new_mst_version.set_minor(8),
            MstVersionKnown::V170 => new_mst_version.set_minor(7),
            MstVersionKnown::V160 => new_mst_version.set_minor(6),
        };

        Self {
            version: new_mst_version,
            header: self.header,
            entries: self.entries.convert(new_version),
        }
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
