use std::ffi::CString;
use std::ffi::CStr;

pub type ConstCCharPtr = *const libc::c_char;
pub type Size = libc::size_t;

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