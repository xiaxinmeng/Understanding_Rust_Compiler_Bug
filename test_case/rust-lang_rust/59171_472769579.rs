rust
#![feature(lang_items)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(a: &PanicInfo) -> ! { 
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {}
