use binrw::{BinRead, BinWrite};

use super::entry::*;
use super::header::MstHeader;
use super::MstVersion;

pub type CanonicalInnerEntries = InnerEntries<CanonicalEntry, CanonicalSupportEntry>;

#[derive(BinRead, BinWrite, Debug)]
#[br(import(header: MstHeader))]
#[bw(import(entry_offsets: EntryOffsets))]
pub struct InnerEntries<E, S>
where
    E: Entry + BinRead<Args = ()> + BinWrite<Args = (Option<EntryOffsets>,)> + Clone,
    S: SupportEntry + BinRead<Args = ()> + BinWrite<Args = ()> + Clone,
{
    #[br(count = header.num_entries)]
    #[bw(args(Some(entry_offsets)))]
    pub entries: Vec<E>,
    #[br(count = header.num_free_entries)]
    #[bw(args(None))]
    pub free_entries: Vec<E>,
    #[br(count = header.num_support_entries)]
    pub support_entries: Vec<S>,
    #[br(count = header.num_free_support_entries)]
    pub free_support_entries: Vec<S>,
}

impl<
        E: Into<CanonicalEntry>
            + Entry
            + BinRead<Args = ()>
            + BinWrite<Args = (Option<EntryOffsets>,)>
            + Clone,
        S: Into<CanonicalSupportEntry>
            + SupportEntry
            + BinRead<Args = ()>
            + BinWrite<Args = ()>
            + Clone,
    > InnerEntries<E, S>
{
    pub fn into_canonical(&self) -> CanonicalInnerEntries {
        CanonicalInnerEntries {
            entries: self
                .entries
                .clone()
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
            free_entries: self
                .free_entries
                .clone()
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
            support_entries: self
                .support_entries
                .clone()
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
            free_support_entries: self
                .free_support_entries
                .clone()
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
        }
    }
}

impl<
        E: From<CanonicalEntry>
            + Entry
            + BinRead<Args = ()>
            + BinWrite<Args = (Option<EntryOffsets>,)>
            + Clone,
        S: From<CanonicalSupportEntry>
            + SupportEntry
            + BinRead<Args = ()>
            + BinWrite<Args = ()>
            + Clone,
    > InnerEntries<E, S>
{
    pub fn from_canonical(canonical: CanonicalInnerEntries) -> Self {
        Self {
            entries: canonical
                .entries
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
            free_entries: canonical
                .free_entries
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
            support_entries: canonical
                .support_entries
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
            free_support_entries: canonical
                .free_support_entries
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(BinRead, BinWrite, Debug)]
#[br(import(version: MstVersion, header: MstHeader))]
#[bw(import(version: MstVersion, entry_offsets: EntryOffsets))]
pub enum Entries {
    #[br(pre_assert(version.major() == 1 && version.minor() == 8 && version.ps2() > 0))]
    V180PS2 {
        #[br(args(header))]
        #[bw(
            args(entry_offsets),
            assert(
                version.major() == 1 && version.minor() == 8 && version.ps2() > 0,
                "MstVersion {}.{}.{} does not match Entries version 1.8.0 (PS2)",
                version.major(), version.minor(), version.patch(),
            )
        )]
        inner: InnerEntries<EntryV180Variable<20>, SupportEntryVariable<20>>,
    },
    #[br(pre_assert(version.major() == 1 && version.minor() == 8))]
    V180 {
        #[br(args(header))]
        #[bw(
            args(entry_offsets),
            assert(
                version.major() == 1 && version.minor() == 8 && version.ps2() == 0,
                "MstVersion {}.{}.{} does not match Entries version 1.8.0",
                version.major(), version.minor(), version.patch(),
            )
        )]
        inner: InnerEntries<EntryV180Variable<16>, SupportEntryVariable<16>>,
    },
    #[br(pre_assert(version.major() == 1 && version.minor() == 7))]
    V170 {
        #[br(args(header))]
        #[bw(
            args(entry_offsets),
            assert(
                version.major() == 1 && version.minor() == 7,
                "MstVersion {}.{}.{} does not match Entries version 1.7.0",
                version.major(), version.minor(), version.patch(),
            )
        )]
        inner: InnerEntries<EntryV170, SupportEntryVariable<16>>,
    },
    #[br(pre_assert(version.major() == 1 && version.minor() == 6))]
    V160 {
        #[br(args(header))]
        #[bw(
            args(entry_offsets),
            assert(
                version.major() == 1 && version.minor() == 6,
                "MstVersion {}.{}.{} does not match Entries version 1.6.0",
                version.major(), version.minor(), version.patch(),
            )
        )]
        inner: InnerEntries<EntryV160, SupportEntryVariable<16>>,
    },
}

impl Entries {
    pub fn convert(&self, major: u8, minor: u8, patch: u8, ps2: bool) -> anyhow::Result<Entries> {
        Ok(match (self, major, minor, patch, ps2) {
            (Entries::V180PS2 { .. }, 1, 8, 0, true) => {
                anyhow::bail!("Invalid Mst Entry conversion (self)")
            }
            (Entries::V180 { .. }, 1, 8, 0, false) => {
                anyhow::bail!("Invalid Mst Entry conversion (self)")
            }
            (Entries::V170 { .. }, 1, 7, 0, false) => {
                anyhow::bail!("Invalid Mst Entry conversion (self)")
            }
            (Entries::V160 { .. }, 1, 6, 0, false) => {
                anyhow::bail!("Invalid Mst Entry conversion (self)")
            }

            (Entries::V180 { inner }, 1, 8, 0, true) => Entries::V180PS2 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V170 { inner }, 1, 8, 0, true) => Entries::V180PS2 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V160 { inner }, 1, 8, 0, true) => Entries::V180PS2 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },

            (Entries::V180PS2 { inner }, 1, 8, 0, false) => Entries::V180 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V170 { inner }, 1, 8, 0, false) => Entries::V180 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V160 { inner }, 1, 8, 0, false) => Entries::V180 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },

            (Entries::V180PS2 { inner }, 1, 7, 0, false) => Entries::V170 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V180 { inner }, 1, 7, 0, false) => Entries::V170 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V160 { inner }, 1, 7, 0, false) => Entries::V170 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },

            (Entries::V180PS2 { inner }, 1, 6, 0, false) => Entries::V160 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V180 { inner }, 1, 6, 0, false) => Entries::V160 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V170 { inner }, 1, 6, 0, false) => Entries::V160 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            _ => {
                anyhow::bail!("Invalid Mst Entry conversion (unknown)");
            }
        })
    }
}
