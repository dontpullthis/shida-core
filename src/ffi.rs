use std::ffi::CString;
use std::ffi::CStr;
use std::mem;

pub type ConstCCharPtr = *const libc::c_char;
pub type MutCCharPtr = *mut libc::c_char;
pub type Size = libc::size_t;

pub unsafe fn bytes_vec_to_ccharptr(input: &Vec<u8>) -> MutCCharPtr {
    CString::from_vec_unchecked(input.to_vec()).into_raw()
}

pub fn ccharptr_to_bytes_vec(input: ConstCCharPtr) -> Vec<u8> {
    let c_str: &CStr = unsafe { CStr::from_ptr(input) };
    let str_slice: &str = c_str.to_str().unwrap();
    Vec::from(str_slice)
}


pub unsafe fn string_to_ccharptr(string: String) -> ConstCCharPtr {
    CString::from_vec_unchecked(Vec::from(string.as_bytes())).into_raw()
}

pub unsafe fn ccharptr_to_string(ccharptr: ConstCCharPtr) -> Result<String, std::str::Utf8Error> {
    let cstr = CStr::from_ptr(ccharptr);
    
    match cstr.to_str() {
        Ok(s) => Ok(String::from(s)),
        Err(e) => Err(e),
    }
}

pub unsafe fn malloc<T>(len: usize) -> *mut T {
    libc::malloc(mem::size_of::<T>() * len) as *mut T
}