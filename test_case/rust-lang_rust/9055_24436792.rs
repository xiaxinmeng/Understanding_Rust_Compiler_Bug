 Rust
#[crate_type = "lib"];

use std::ptr;

pub use std::libc::*;
pub use std::libc::types::os::arch::extra::*;

extern "stdcall" {
    fn MessageBoxW(hWnd: HANDLE, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: c_uint) -> c_int;
}

pub fn g<T>() {
    #[fixed_stack_segment];
    unsafe { MessageBoxW(ptr::mut_null(), ptr::null(), ptr::null(), 0 as c_uint); }
}
