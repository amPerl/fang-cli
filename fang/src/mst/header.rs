use binrw::{BinRead, BinWrite};

#[derive(BinRead, BinWrite, Debug, Clone, Copy)]
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
