rust
// src/main.rs:
#![no_main]
#![no_std]

use core::ffi::c_void;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn main(_h: *mut c_void, _st: *mut c_void) -> usize {
    log::error!("hello");

    0
}
