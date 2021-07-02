use crate::ffi::typedefs;

#[repr(C)]
pub struct Error {
    pub code: typedefs::Size,
    pub message: typedefs::ConstCCharPtr,
}
