 rust
#![feature(lang_items)]
#![no_std]

use core::ptr;

#[no_mangle]
pub unsafe fn _start() {
    extern "C" {
        fn main() -> !;
    }

    ptr::read_volatile(_start as *const usize);

    main();
}

// stubs
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
