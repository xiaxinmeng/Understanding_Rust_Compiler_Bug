rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn KernelMain() {
    for _ in 0..1 {}
}

