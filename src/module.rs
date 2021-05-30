#[allow(unused_imports)]
type CanHandleFunc = fn(connection_type: *const libc::c_char) -> bool;

pub struct Module {
    pub can_handle: CanHandleFunc,
}