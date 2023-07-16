console
> cat .\src\main.rs
#![feature(panic_handler)]
#![no_main]
#![no_std]

#[no_mangle]
pub fn _start() -> ! {
    panic!()
}

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
