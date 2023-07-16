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

// linkage to CRT library according to crt-static flag set in .cargo/config:
// [target.x86_64-pc-windows-msvc]
// rustflags = ["-C", "target-feature=+crt-static"]
#[cfg(target_feature = "crt-static")]
#[link(name = "libcmt")]
extern {}
#[cfg(not(target_feature = "crt-static"))]
#[link(name = "msvcrt")]
extern {}

// CRT is used, the entry point is main
#[no_mangle]
pub extern "C" fn main() -> i32 {
    panic!();
}
