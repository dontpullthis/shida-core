use std::ffi::CString;
use std::ffi::CStr;
use std::str::Utf8Error;

use crate::ffi::typedefs;
use crate::ffi::utils;

pub fn bytes_vec_to_ccharptr(input: &Vec<u8>) -> typedefs::MutCCharPtr {
    unsafe { CString::from_vec_unchecked(input.to_vec()).into_raw() }
}

pub fn ccharptr_to_bytes_vec(input: typedefs::ConstCCharPtr) -> Vec<u8> {
    let c_str: &CStr = unsafe { CStr::from_ptr(input) };
    let str_slice: &str = c_str.to_str().unwrap();
    Vec::from(str_slice)
}

pub fn str_to_ccharptr(string: &str) -> typedefs::MutCCharPtr {
    unsafe { CString::from_vec_unchecked(Vec::from(string)).into_raw() }
}

pub fn string_to_ccharptr(string: String) -> typedefs::MutCCharPtr {
    unsafe { CString::from_vec_unchecked(Vec::from(string.as_bytes())).into_raw() }
}

pub fn ccharptr_to_string(ccharptr: typedefs::ConstCCharPtr) -> Result<String, std::str::Utf8Error> {
    let cstr = unsafe { CStr::from_ptr(ccharptr) };
    
    match cstr.to_str() {
        Ok(s) => Ok(String::from(s)),
        Err(e) => Err(e),
    }
}

pub fn string_vec_to_cchar_ptr(input: &Vec<String>) -> *mut typedefs::MutCCharPtr {
    let size = input.len();
    let result = utils::malloc::<typedefs::MutCCharPtr>(size);
    for (index, kv) in input.iter().enumerate() {
        unsafe {
            let val = result.offset(index as isize);
            *val = str_to_ccharptr(kv.as_str());
        }
    }
    result
}

pub fn cchar_ptr_to_vec_string(paramsc: typedefs::Size, paramsv: *const typedefs::ConstCCharPtr) -> Result<Vec<String>, Utf8Error> {
    let mut result = Vec::new();
    for i in 0..paramsc {
        let ch: typedefs::ConstCCharPtr = unsafe { *paramsv.offset(i as isize) };
        let param = match ccharptr_to_string(ch) {
            Ok(string) => string,
            Err(e) => return Err(e),
        };
        result.push(param);
    }
    Ok(result)
}