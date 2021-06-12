use std::ffi::CString;
use std::ffi::CStr;
use std::mem;

pub type ConstCCharPtr = *const libc::c_char;
pub type MutCCharPtr = *mut libc::c_char;
pub type Size = libc::size_t;

pub fn bytes_vec_to_ccharptr(input: &Vec<u8>) -> MutCCharPtr {
    unsafe { CString::from_vec_unchecked(input.to_vec()).into_raw() }
}

pub fn ccharptr_to_bytes_vec(input: ConstCCharPtr) -> Vec<u8> {
    let c_str: &CStr = unsafe { CStr::from_ptr(input) };
    let str_slice: &str = c_str.to_str().unwrap();
    Vec::from(str_slice)
}


pub fn str_to_ccharptr(string: &str) -> MutCCharPtr {
    unsafe { CString::from_vec_unchecked(Vec::from(string)).into_raw() }
}

pub fn string_to_ccharptr(string: String) -> MutCCharPtr {
    unsafe { CString::from_vec_unchecked(Vec::from(string.as_bytes())).into_raw() }
}

pub fn ccharptr_to_string(ccharptr: ConstCCharPtr) -> Result<String, std::str::Utf8Error> {
    let cstr = unsafe { CStr::from_ptr(ccharptr) };
    
    match cstr.to_str() {
        Ok(s) => Ok(String::from(s)),
        Err(e) => Err(e),
    }
}

pub fn string_vec_to_cchar_ptr(input: &Vec<String>) -> *mut MutCCharPtr {
    let size = input.len();
    let result = malloc::<MutCCharPtr>(size);
    for (index, kv) in input.iter().enumerate() {
        unsafe {
            let val = result.offset(index as isize);
            *val = str_to_ccharptr(kv.as_str());
        }
    }
    result
}

pub fn malloc<T>(len: usize) -> *mut T {
    unsafe { libc::malloc(mem::size_of::<T>() * len) as *mut T }
}