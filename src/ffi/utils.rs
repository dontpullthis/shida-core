use std::mem;

pub fn malloc<T>(len: usize) -> *mut T {
    unsafe { libc::malloc(mem::size_of::<T>() * len) as *mut T }
}