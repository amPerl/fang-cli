use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom, Write},
    rc::Rc,
};

use binrw::BinWriterExt;
use chrono::Utc;

use super::{
    entries::{Entries, InnerEntries},
    entry::{EntryV160, EntryV170, EntryV180Variable},
    header::{MstCompilers, MstHeader},
    Mst, MstBody, MstIdentifier, MstPlatformKnown, MstVersion, MstVersionKnown,
};

#[derive(Debug)]
pub struct MstBuilder {
    platform: MstPlatformKnown,
    version: MstVersionKnown,
    compilers: MstCompilers,
    entry_sources: HashMap<String, MstBuilderEntrySource>,
}

impl MstBuilder {
    pub fn new(
        version: MstVersionKnown,
        platform: MstPlatformKnown,
        compilers: MstCompilers,
    ) -> Self {
        Self {
            platform,
            version,
            compilers,
            entry_sources: Default::default(),
        }
    }

    /// Create a builder with no entries but with versions and platform copied from an existing Mst
    pub fn from_mst_empty(mst: &Mst) -> anyhow::Result<Self> {
        Ok(Self {
            platform: MstPlatformKnown::try_from(mst.body.version())?,
            version: MstVersionKnown::try_from(mst.body.version())?,
            compilers: mst.body.header.compilers,
            entry_sources: Default::default(),
        })
    }

    // Update the target Mst version
    pub fn set_version(&mut self, version: &MstVersionKnown) {
        self.version = *version;
    }

    /// Check if an entry by the specified path has already been added
    pub fn has_entry(&self, path: String) -> bool {
        self.entry_sources.contains_key(&path)
    }

    /// Add an entry from memory
    pub fn add_entry_memory(&mut self, path: String, data: Vec<u8>, timestamp: Option<u32>) {
        self.entry_sources
            .insert(path, MstBuilderEntrySource::Memory { data, timestamp });
    }

    /// Add an entry that will be read from a file
    pub fn add_entry_file(
        &mut self,
        entry_path: String,
        file_path: String,
        offset: usize,
        size: usize,
        timestamp: Option<u32>,
    ) {
        self.entry_sources.insert(
            entry_path,
            MstBuilderEntrySource::File {
                path: file_path,
                offset,
                size,
                timestamp,
            },
        );
    }

    /// Creates the table of entries for the specific target version of Mst
    fn create_entries(&self) -> Entries {
        let timestamp_now = Utc::now().timestamp() as u32;

        match self.version {
            MstVersionKnown::V180PS2 => Entries::V180PS2 {
                inner: InnerEntries {
                    entries: self
                        .entry_sources
                        .iter()
                        .map(|(path, source)| EntryV180Variable {
                            filename: path.into(),
                            flags: 0,
                            offset: 0,
                            size: match source {
                                MstBuilderEntrySource::File { size, .. } => *size as u32,
                                MstBuilderEntrySource::Memory { data, .. } => data.len() as u32,
                            },
                            timestamp: match source {
                                MstBuilderEntrySource::File { timestamp, .. } => {
                                    timestamp.unwrap_or(timestamp_now)
                                }
                                MstBuilderEntrySource::Memory { timestamp, .. } => {
                                    timestamp.unwrap_or(timestamp_now)
                                }
                            },
                            crc: 0,
                        })
                        .collect(),
                    free_entries: Vec::new(),
                    support_entries: Vec::new(),
                    free_support_entries: Vec::new(),
                },
            },
            MstVersionKnown::V180 => Entries::V180 {
                inner: InnerEntries {
                    entries: self
                        .entry_sources
                        .iter()
                        .map(|(path, source)| EntryV180Variable {
                            filename: path.into(),
                            flags: 0,
                            offset: 0,
                            size: match source {
                                MstBuilderEntrySource::File { size, .. } => *size as u32,
                                MstBuilderEntrySource::Memory { data, .. } => data.len() as u32,
                            },
                            timestamp: match source {
                                MstBuilderEntrySource::File { timestamp, .. } => {
                                    timestamp.unwrap_or(timestamp_now)
                                }
                                MstBuilderEntrySource::Memory { timestamp, .. } => {
                                    timestamp.unwrap_or(timestamp_now)
                                }
                            },
                            crc: 0,
                        })
                        .collect(),
                    free_entries: Vec::new(),
                    support_entries: Vec::new(),
                    free_support_entries: Vec::new(),
                },
            },
            MstVersionKnown::V170 => Entries::V170 {
                inner: InnerEntries {
                    entries: self
                        .entry_sources
                        .iter()
                        .map(|(path, source)| EntryV170 {
                            filename: path.into(),
                            offset: 0,
                            size: match source {
                                MstBuilderEntrySource::File { size, .. } => *size as u32,
                                MstBuilderEntrySource::Memory { data, .. } => data.len() as u32,
                            },
                            timestamp: match source {
                                MstBuilderEntrySource::File { timestamp, .. } => {
                                    timestamp.unwrap_or(timestamp_now)
                                }
                                MstBuilderEntrySource::Memory { timestamp, .. } => {
                                    timestamp.unwrap_or(timestamp_now)
                                }
                            },
                            crc: 0,
                        })
                        .collect(),
                    free_entries: Vec::new(),
                    support_entries: Vec::new(),
                    free_support_entries: Vec::new(),
                },
            },
            MstVersionKnown::V160 => Entries::V160 {
                inner: InnerEntries {
                    entries: self
                        .entry_sources
                        .iter()
                        .map(|(path, source)| EntryV160 {
                            filename: path.into(),
                            offset: 0,
                            size: match source {
                                MstBuilderEntrySource::File { size, .. } => *size as u32,
                                MstBuilderEntrySource::Memory { data, .. } => data.len() as u32,
                            },
                            timestamp: match source {
                                MstBuilderEntrySource::File { timestamp, .. } => {
                                    timestamp.unwrap_or(timestamp_now)
                                }
                                MstBuilderEntrySource::Memory { timestamp, .. } => {
                                    timestamp.unwrap_or(timestamp_now)
                                }
                            },
                        })
                        .collect(),
                    free_entries: Vec::new(),
                    support_entries: Vec::new(),
                    free_support_entries: Vec::new(),
                },
            },
        }
    }

    /// Create the Mst structure of specific target version
    fn create_mst(&self) -> Mst {
        let identifier = MstIdentifier::from_known(self.platform);
        let version = MstVersion::from_known(self.version, self.platform);
        let entries = self.create_entries();

        Mst {
            identifier,
            body: MstBody {
                version,
                header: MstHeader {
                    bytes_in_file: Default::default(),
                    num_entries: self.entry_sources.len() as u32,
                    num_free_entries: 0,
                    num_support_entries: 0,
                    num_free_support_entries: 0,
                    data_offset: Default::default(),
                    compilers: self.compilers,
                    reserved: Default::default(),
                },
                entries,
            },
        }
    }

    pub fn write<W: Write + Seek>(self, writer: &mut W) -> anyhow::Result<()> {
        let mst = self.create_mst();

        // Write the Mst header with entries to the output file, recording where we should later place the offsets to each entry's data
        let content_offset_offsets = Rc::new(RefCell::new(Vec::new()));
        writer.write_le_args(&mst, (content_offset_offsets.clone(),))?;

        // Write each entry's data to the output file, recording their offsets
        let mut content_offsets = Vec::new();
        for (_entry_path, source) in self.entry_sources {
            let mut pos = writer.stream_position()?;

            // Align the entry's data to 2048 bytes if necessary
            if pos % 2048 > 0 {
                let padding = 2048 - pos % 2048;
                let padding = vec![0u8; padding as usize];
                writer.write_all(&padding)?;
                pos = writer.stream_position()?;
            }

            // Read the entry's data from the source if necessary
            let content_buf = match source {
                MstBuilderEntrySource::File {
                    path: source_path,
                    offset,
                    size,
                    ..
                } => {
                    let mut source_file = BufReader::new(File::open(&source_path)?);
                    source_file.seek(SeekFrom::Start(offset as u64))?;
                    let mut data = vec![0u8; size];
                    source_file.read_exact(&mut data)?;
                    data
                }
                MstBuilderEntrySource::Memory { data, .. } => data,
            };

            // Write the data to the output file
            content_offsets.push(pos);
            writer.write_all(&content_buf)?;
        }

        let file_size = writer.stream_position()?;

        // Write the total size in bytes into the Mst header
        writer.seek(SeekFrom::Start(8))?;
        let total_size_bytes = file_size as u32;
        if mst.identifier.is_little() {
            writer.write_all(&total_size_bytes.to_le_bytes())?;
        } else {
            writer.write_all(&total_size_bytes.to_be_bytes())?;
        }

        // Write the data offset into the Mst header
        // If we have entries (as we should), use the lowest offset recorded
        // Otherwise, use the file size, which should be just the length of the header
        writer.seek(SeekFrom::Start(28))?;
        let data_offset = *content_offsets.iter().min().unwrap_or(&file_size) as u32;
        if mst.identifier.is_little() {
            writer.write_all(&data_offset.to_le_bytes())?;
        } else {
            writer.write_all(&data_offset.to_be_bytes())?;
        }

        // Write the data offsets for every entry in the Mst header
        for (pos, entry_offset) in content_offset_offsets.borrow().iter().zip(content_offsets) {
            writer.seek(SeekFrom::Start(*pos))?;

            let entry_offset = entry_offset as u32;
            if mst.identifier.is_little() {
                writer.write_all(&entry_offset.to_le_bytes())?;
            } else {
                writer.write_all(&entry_offset.to_be_bytes())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
enum MstBuilderEntrySource {
    /// The entry will be read from a file, at the specified position and amount of bytes
    File {
        path: String,
        offset: usize,
        size: usize,
        timestamp: Option<u32>,
    },
    /// The entry will be dumped from memory
    Memory {
        data: Vec<u8>,
        timestamp: Option<u32>,
    },
}
