use binrw::BinRead;

#[derive(BinRead)]
pub struct CFVec2 {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Debug for CFVec2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("CFVec2({}, {})", self.x, self.y))
    }
}

#[derive(BinRead)]
pub struct CFVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl std::fmt::Debug for CFVec3 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("CFVec3({}, {}, {})", self.x, self.y, self.z))
    }
}

#[derive(BinRead)]
pub struct CFMtx43 {
    pub x: CFVec3,
    pub y: CFVec3,
    pub z: CFVec3,
}

impl std::fmt::Debug for CFMtx43 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.debug_tuple("CFMtx43")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

#[derive(BinRead)]
pub struct CFVec3A {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl std::fmt::Debug for CFVec3A {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!(
            "CFVec3A({}, {}, {}, {})",
            self.x, self.y, self.z, self.w
        ))
    }
}

#[derive(BinRead)]
pub struct CFMtx43A {
    pub x: CFVec3A,
    pub y: CFVec3A,
    pub z: CFVec3A,
    pub p: CFVec3A,
}

impl std::fmt::Debug for CFMtx43A {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.debug_tuple("CFMtx43A")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.p)
            .finish()
    }
}

#[derive(BinRead, Debug)]
pub struct CFSphere {
    pub radius: f32,
    pub pos: CFVec3,
}
