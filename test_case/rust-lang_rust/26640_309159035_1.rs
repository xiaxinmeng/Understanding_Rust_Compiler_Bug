rust
#![feature(lang_items)]
#![no_std]
extern crate meal;

#[lang="panic_fmt"]
extern fn panic_fmt(_msg: &core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop { }
}

#[lang="eh_personality"]
extern fn eh_personality() {
    loop { }
}

#[no_mangle]
pub extern fn libdiner_actually_public_function() -> i32 {
    17
}
