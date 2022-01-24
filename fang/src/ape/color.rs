use binrw::BinRead;

#[derive(BinRead, Debug)]
pub struct CFColorRGB {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(BinRead, Debug)]
pub struct CFColorRGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(BinRead, Debug)]
pub struct CFColorMotif {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub motif_index: u32,
}
