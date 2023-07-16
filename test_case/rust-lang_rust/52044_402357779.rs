
#![no_std]
#![feature(panic_implementation)]

use core::panic::PanicInfo;

#[panic_implementation]
fn rust_begin_panic(_info: &PanicInfo) -> ! {
   loop {}
}

#[no_mangle]
pub extern fn foo() {
    panic!(\"foo\");
}
