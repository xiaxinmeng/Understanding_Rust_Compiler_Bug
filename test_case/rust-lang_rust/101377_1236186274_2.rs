rust
// src/main.rs:
#![no_main]
#![no_std]

use uefi::prelude::*;

#[export_name = "efi_main"]
pub extern "C" fn main(_h: *mut core::ffi::c_void, mut st: SystemTable<Boot>) -> usize {
    uefi_services::init(&mut st).expect("Failed to initialize utilities");

    0
}
