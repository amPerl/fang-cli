use interoptopus::{ffi_type, patterns::result::FFIError};

#[ffi_type(patterns(ffi_error))]
#[repr(C)]
pub enum FangFFIResult {
    Ok = 0,
    NullPassed = 1,
    Panic = 2,
    OtherError = 3,
}

impl FFIError for FangFFIResult {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::NullPassed;
    const PANIC: Self = Self::Panic;
}

impl From<anyhow::Error> for FangFFIResult {
    fn from(_err: anyhow::Error) -> Self {
        Self::OtherError
    }
}
