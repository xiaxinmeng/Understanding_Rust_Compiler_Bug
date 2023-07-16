Rust
#![cfg(target_os = "windows")]
#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::panic::PanicInfo;

// required by no_std
#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memset(mem: *mut u8, _val: i32, _n: usize) -> *mut u8 {
    mem
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, _src: *const u8, _n: usize) -> *mut u8 {
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(_mem1: *const u8, _mem2: *const u8, _n: usize) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn __CxxFrameHandler3() {
}

// no CRT and console subsystem => the entry point is mainCRTStartup
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    panic!();
}
