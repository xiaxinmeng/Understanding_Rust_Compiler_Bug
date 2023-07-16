 rust
#![feature(libc)]

extern crate libc;

use std::ptr;
use std::ffi::CString;

pub fn with_c_strings<T: Copy+Into<Vec<u8>>, U, F>(ts: &[T], f: F) -> U where
    F: FnOnce(*const *const libc::c_char) -> U {
    let c_strs: Vec<_> = ts.iter().map(|&t| {
        CString::new(t).unwrap()
    }).collect();
    let ptrs: Vec<_> = c_strs.iter().map(|c_str| c_str.as_ptr())
                     .chain(Some(ptr::null()))
                     .collect();

    f(ptrs.as_ptr())
}
