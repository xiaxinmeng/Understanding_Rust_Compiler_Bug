 rust
#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub unsafe fn _start() {
    extern "C" {
        fn main() -> !;
    }

    main();
}

// stubs
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
