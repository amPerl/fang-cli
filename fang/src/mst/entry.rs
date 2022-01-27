use binrw::{BinRead, BinWrite, WriteOptions};
use chrono::{DateTime, Utc};
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::SeekFrom;
use std::rc::Rc;

use super::{MstHeader, MstVersion};

#[derive(BinRead, BinWrite, Clone)]
pub struct Filename<const T: usize>([u8; T]);

impl<const T: usize> Filename<T> {
    pub fn is_empty(&self) -> bool {
        T == 0 || self.0[0] == 0
    }
}

impl<const T: usize> ToString for Filename<T> {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .take_while(|b| **b > 0)
            .flat_map(|b| char::from_u32(*b as u32))
            .collect()
    }
}

impl<const T: usize> Debug for Filename<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        String::fmt(&self.to_string(), f)
    }
}

impl From<Filename<16>> for Filename<20> {
    fn from(other: Filename<16>) -> Self {
        let mut new_buf = [0u8; 20];
        new_buf.split_at_mut(16).0.copy_from_slice(&other.0);
        Filename(new_buf)
    }
}

impl From<Filename<20>> for Filename<16> {
    fn from(other: Filename<20>) -> Self {
        let mut new_buf = [0u8; 16];
        new_buf.split_at_mut(15).0.copy_from_slice(&other.0[..15]);
        Filename(new_buf)
    }
}

pub trait Entry {
    fn filename(&self) -> String;
    fn offset(&self) -> usize;
    fn size(&self) -> usize;
    fn timestamp(&self) -> DateTime<Utc>;
}

pub trait SupportEntry {
    fn filename(&self) -> String;
    fn timestamp(&self) -> DateTime<Utc>;
}

pub type EntryOffsets = Rc<RefCell<Vec<u64>>>;

pub type CanonicalEntry = EntryV180Variable<20>;
pub type CanonicalSupportEntry = SupportEntryVariable<20>;
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

#[derive(BinRead, BinWrite, Debug, Clone)]
#[bw(import(entry_offsets: Option<EntryOffsets>))]
pub struct EntryV180Variable<const FNL: usize> {
    pub filename: Filename<FNL>,
    #[brw(pad_after = 2)]
    pub flags: u16,
    #[bw(args(entry_offsets), write_with = record_value)]
    pub offset: u32,
    pub size: u32,
    pub timestamp: u32,
    pub crc: u32,
}

impl<const T: usize> Entry for EntryV180Variable<T> {
    fn filename(&self) -> String {
        self.filename.to_string()
    }

    fn offset(&self) -> usize {
        self.offset as usize
    }

    fn size(&self) -> usize {
        self.size as usize
    }

    fn timestamp(&self) -> DateTime<Utc> {
        crate::util::epoch_to_chrono(self.timestamp)
    }
}

impl From<EntryV180Variable<16>> for CanonicalEntry {
    fn from(other: EntryV180Variable<16>) -> Self {
        Self {
            filename: other.filename.into(),
            flags: other.flags,
            offset: other.offset,
            size: other.size,
            timestamp: other.timestamp,
            crc: other.crc,
        }
    }
}

impl From<CanonicalEntry> for EntryV180Variable<16> {
    fn from(other: CanonicalEntry) -> Self {
        Self {
            filename: other.filename.into(),
            flags: other.flags,
            offset: other.offset,
            size: other.size,
            timestamp: other.timestamp,
            crc: other.crc,
        }
    }
}

#[derive(BinRead, BinWrite, Debug, Clone)]
#[bw(import(entry_offsets: Option<EntryOffsets>))]
pub struct EntryV170 {
    pub filename: Filename<16>,
    #[bw(args(entry_offsets), write_with = record_value)]
    pub offset: u32,
    pub size: u32,
    pub timestamp: u32,
    pub crc: u32,
}

impl Entry for EntryV170 {
    fn filename(&self) -> String {
        self.filename.to_string()
    }

    fn offset(&self) -> usize {
        self.offset as usize
    }

    fn size(&self) -> usize {
        self.size as usize
    }

    fn timestamp(&self) -> DateTime<Utc> {
        crate::util::epoch_to_chrono(self.timestamp)
    }
}

impl From<EntryV170> for CanonicalEntry {
    fn from(other: EntryV170) -> Self {
        Self {
            filename: other.filename.into(),
            flags: 0,
            offset: other.offset,
            size: other.size,
            timestamp: other.timestamp,
            crc: other.crc,
        }
    }
}

impl From<CanonicalEntry> for EntryV170 {
    fn from(other: CanonicalEntry) -> Self {
        Self {
            filename: other.filename.into(),
            offset: other.offset,
            size: other.size,
            timestamp: other.timestamp,
            crc: other.crc,
        }
    }
}

#[derive(BinRead, BinWrite, Debug, Clone)]
#[bw(import(entry_offsets: Option<EntryOffsets>))]
pub struct EntryV160 {
    pub filename: Filename<16>,
    #[bw(args(entry_offsets), write_with = record_value)]
    pub offset: u32,
    pub size: u32,
    pub timestamp: u32,
}

impl Entry for EntryV160 {
    fn filename(&self) -> String {
        self.filename.to_string()
    }

    fn offset(&self) -> usize {
        self.offset as usize
    }

    fn size(&self) -> usize {
        self.size as usize
    }

    fn timestamp(&self) -> DateTime<Utc> {
        crate::util::epoch_to_chrono(self.timestamp)
    }
}

impl From<EntryV160> for CanonicalEntry {
    fn from(other: EntryV160) -> Self {
        Self {
            filename: other.filename.into(),
            flags: 0,
            offset: other.offset,
            size: other.size,
            timestamp: other.timestamp,
            crc: 0,
        }
    }
}

impl From<CanonicalEntry> for EntryV160 {
    fn from(other: CanonicalEntry) -> Self {
        Self {
            filename: other.filename.into(),
            offset: other.offset,
            size: other.size,
            timestamp: other.timestamp,
        }
    }
}

fn record_value<W: binrw::io::Write + binrw::io::Seek>(
    &value: &u32,
    writer: &mut W,
    opts: &WriteOptions,
    args: (Option<EntryOffsets>,),
) -> binrw::BinResult<()> {
    if let Some(entry_offsets) = args.0 {
        let pos = writer.seek(SeekFrom::Current(0))?;
        entry_offsets.borrow_mut().push(pos);
    }
    value.write_options(writer, opts, ())
}

#[derive(BinRead, BinWrite, Debug, Clone)]
pub struct SupportEntryVariable<const FNL: usize> {
    pub filename: Filename<FNL>,
    pub timestamp: u32,
}

impl<const T: usize> SupportEntry for SupportEntryVariable<T> {
    fn filename(&self) -> String {
        self.filename.to_string()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        crate::util::epoch_to_chrono(self.timestamp)
    }
}

impl From<SupportEntryVariable<16>> for CanonicalSupportEntry {
    fn from(other: SupportEntryVariable<16>) -> Self {
        Self {
            filename: other.filename.into(),
            timestamp: other.timestamp,
        }
    }
}

impl From<CanonicalSupportEntry> for SupportEntryVariable<16> {
    fn from(other: CanonicalSupportEntry) -> Self {
        Self {
            filename: other.filename.into(),
            timestamp: other.timestamp,
        }
    }
}
