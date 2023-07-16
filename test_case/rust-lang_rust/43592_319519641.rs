rust
extern crate libc;

use std::ffi::CStr;
use std::mem;

fn glibc_version() -> Option<&'static CStr> {
    unsafe {
        let funcptr = libc::dlsym(
            libc::RTLD_DEFAULT,
            "gnu_get_libc_version\0".as_ptr() as *const libc::c_char,
        );
        if !funcptr.is_null() {
            type CharstarFn = extern "C" fn() -> *const libc::c_char;
            let gnu_get_libc_version: CharstarFn = mem::transmute(funcptr);
            Some(CStr::from_ptr(gnu_get_libc_version()))
        } else {
            None
        }
    }
}

fn main() {
    println!("{:?}", glibc_version());
}
