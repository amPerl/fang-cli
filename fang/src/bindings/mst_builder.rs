use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::AsciiPointer;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};

use super::error::FangFFIResult;
use crate::mst::{header::MstCompilers, MstPlatformKnown, MstVersionKnown};

#[ffi_type(opaque)]
pub struct MstBuilder {
    inner: crate::mst::builder::MstBuilder,
}

#[ffi_service(error = "FangFFIResult", prefix = "mst_builder_")]
impl MstBuilder {
    #[ffi_service_ctor]
    pub fn new(
        version: MstVersionKnown,
        platform: MstPlatformKnown,
        compilers: MstCompilers,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            inner: crate::mst::builder::MstBuilder::new(version, platform, compilers),
        })
    }

    #[ffi_service_method(on_panic = "ffi_error")]
    pub fn add_entry_memory(
        &mut self,
        path: AsciiPointer,
        data: FFISlice<u8>,
        timestamp: u32,
    ) -> anyhow::Result<()> {
        self.inner.add_entry_memory(
            path.as_str()?.into(),
            data.as_slice(),
            if timestamp == 0 {
                None
            } else {
                Some(timestamp)
            },
        );
        Ok(())
    }
}
