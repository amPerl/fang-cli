use binrw::{BinRead, BinWrite, WriteOptions};
use chrono::{DateTime, Utc};
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::SeekFrom;
use std::rc::Rc;

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

impl<const T: usize> From<&String> for Filename<T> {
    fn from(other: &String) -> Self {
        let mut new_buf = [0u8; T];
        let max_length = (T - 1).min(other.len());

        let other_trimmed = other.split_at(max_length).0.as_bytes();

        new_buf
            .split_at_mut(max_length)
            .0
            .copy_from_slice(other_trimmed);

        Filename(new_buf)
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

#[derive(BinRead, BinWrite, Debug, Clone)]
#[bw(import(entry_offsets: Option<EntryOffsets>))]
pub struct EntryV180Variable<const FNL: usize> {
    pub filename: Filename<FNL>,
    #[brw(pad_after = 2)]
    pub flags: u16,
    #[bw(args(entry_offsets), write_with = record_entry_offset)]
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
    #[bw(args(entry_offsets), write_with = record_entry_offset)]
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
    #[bw(args(entry_offsets), write_with = record_entry_offset)]
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

fn record_entry_offset<W: binrw::io::Write + binrw::io::Seek>(
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
