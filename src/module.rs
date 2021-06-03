use crate::ffi::{ConstCCharPtr, Size};

#[allow(unused_imports)]
pub type CanHandleFunc = fn(connection_type: ConstCCharPtr) -> bool;

#[allow(unused_imports)]
type InitReaderFunc = fn(paramsc: Size, paramsv: *const ConstCCharPtr) -> (*const u8, ConstCCharPtr);

#[allow(unused_imports)]
type ReadFunc = fn(conn_ptr: *const u8) -> (ConstCCharPtr, ConstCCharPtr);

#[repr(C)]
pub struct Module {
    pub can_handle: CanHandleFunc,
    pub init_reader: InitReaderFunc,
    pub read: ReadFunc
}