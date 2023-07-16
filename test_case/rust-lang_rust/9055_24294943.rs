 Rust
#[crate_type = "lib"];

pub use std::ptr;
pub use std::libc::*;
pub use std::libc::types::os::arch::extra::*;

extern "stdcall" {
    fn MessageBoxW(hWnd: HANDLE, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: c_uint) -> c_int;
}

pub trait Dummy {}

pub trait Window {
    fn message_box(&self);
}

impl<D: Dummy> Window for D {
    fn message_box(&self) {
        #[fixed_stack_segment];
        unsafe { MessageBoxW(ptr::mut_null(), ptr::null(), ptr::null(), 0 as c_uint); }
    }
}
