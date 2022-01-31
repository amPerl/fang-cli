use binrw::{BinRead, PosValue};
use std::io::SeekFrom;

#[derive(BinRead, Debug)]
pub struct SndInitRdg {
    _struct_start: PosValue<()>,

    pub proj_file_bytes: u32,
    pub proj_file_offset: u32,
    pub pool_file_bytes: u32,
    pub pool_file_offset: u32,
    pub sdir_file_bytes: u32,
    pub sdir_file_offset: u32,

    // #[br(seek_before = SeekFrom::Start(_struct_start.pos + proj_file_offset as u64), count = proj_file_bytes)]
    // pub proj_file: ProjFile,
    // #[br(seek_before = SeekFrom::Start(_struct_start.pos + pool_file_offset as u64), count = pool_file_bytes)]
    // pub pool_file: PoolFile,
    #[br(seek_before = SeekFrom::Start(_struct_start.pos + sdir_file_offset as u64), args(sdir_file_bytes))]
    pub sdir_file: SdirFile,
}

#[derive(BinRead, Debug)]
#[br(import(size_bytes: u32))]
pub struct SdirFile {
    #[br(count = (size_bytes - 4) / (0x20 + 0x28), pad_after = 4)]
    // #[br(count = 3)]
    pub dsps: Vec<SdirFileDsp>,
}

#[derive(BinRead, Debug)]
pub struct SdirFileDsp {
    #[br(pad_after = 2)]
    pub id: u16,
    #[br(pad_after = 4)]
    pub samp_offset: u32,
    #[br(pad_after = 1)]
    pub base_note: u8,
    pub sample_rate: u16,
    pub sample_count: u32,
    pub loop_start: u32,
    pub loop_length: u32,
    pub info_offset: u32,
}
