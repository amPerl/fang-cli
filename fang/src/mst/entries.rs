use binrw::{BinRead, BinWrite};

use super::header::MstHeader;
use super::{entry::*, MstVersionKnown};

pub type CanonicalInnerEntries = InnerEntries<CanonicalEntry, CanonicalSupportEntry>;

#[derive(BinRead, BinWrite, Debug, Clone)]
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

#[derive(BinRead, BinWrite, Debug, Clone)]
#[br(import(version: MstVersionKnown, header: MstHeader))]
#[bw(import(version: MstVersionKnown, entry_offsets: EntryOffsets))]
pub enum Entries {
    #[br(pre_assert(version == MstVersionKnown::V180PS2))]
    V180PS2 {
        #[br(args(header))]
        #[bw(
            args(entry_offsets),
            assert(version == MstVersionKnown::V180PS2, "Mst version {:?} does not match Entries version V180PS2", version)
        )]
        inner: InnerEntries<EntryV180Variable<20>, SupportEntryVariable<20>>,
    },
    #[br(pre_assert(version == MstVersionKnown::V180))]
    V180 {
        #[br(args(header))]
        #[bw(
            args(entry_offsets),
            assert(version == MstVersionKnown::V180, "Mst version {:?} does not match Entries version V180", version)
        )]
        inner: InnerEntries<EntryV180Variable<16>, SupportEntryVariable<16>>,
    },
    #[br(pre_assert(version == MstVersionKnown::V170))]
    V170 {
        #[br(args(header))]
        #[bw(
            args(entry_offsets),
            assert(version == MstVersionKnown::V170, "Mst version {:?} does not match Entries version V170", version)
        )]
        inner: InnerEntries<EntryV170, SupportEntryVariable<16>>,
    },
    #[br(pre_assert(version == MstVersionKnown::V160))]
    V160 {
        #[br(args(header))]
        #[bw(
            args(entry_offsets),
            assert(version == MstVersionKnown::V160, "Mst version {:?} does not match Entries version V160", version)
        )]
        inner: InnerEntries<EntryV160, SupportEntryVariable<16>>,
    },
}

impl Entries {
    pub fn convert(&self, new_version: MstVersionKnown) -> Entries {
        match (self, new_version) {
            (Entries::V180PS2 { inner }, MstVersionKnown::V180PS2) => Entries::V180PS2 {
                inner: (*inner).clone(),
            },
            (Entries::V180 { inner }, MstVersionKnown::V180) => Entries::V180 {
                inner: (*inner).clone(),
            },
            (Entries::V170 { inner }, MstVersionKnown::V170) => Entries::V170 {
                inner: (*inner).clone(),
            },
            (Entries::V160 { inner }, MstVersionKnown::V160) => Entries::V160 {
                inner: (*inner).clone(),
            },

            (Entries::V180PS2 { inner }, MstVersionKnown::V180) => Entries::V180 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V180PS2 { inner }, MstVersionKnown::V170) => Entries::V170 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V180PS2 { inner }, MstVersionKnown::V160) => Entries::V160 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V180 { inner }, MstVersionKnown::V180PS2) => Entries::V180PS2 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V180 { inner }, MstVersionKnown::V170) => Entries::V170 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V180 { inner }, MstVersionKnown::V160) => Entries::V160 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V170 { inner }, MstVersionKnown::V180PS2) => Entries::V180PS2 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V170 { inner }, MstVersionKnown::V180) => Entries::V180 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V170 { inner }, MstVersionKnown::V160) => Entries::V160 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V160 { inner }, MstVersionKnown::V180PS2) => Entries::V180PS2 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V160 { inner }, MstVersionKnown::V180) => Entries::V180 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
            (Entries::V160 { inner }, MstVersionKnown::V170) => Entries::V170 {
                inner: InnerEntries::<_, _>::from_canonical(inner.into_canonical()),
            },
        }
    }
}
