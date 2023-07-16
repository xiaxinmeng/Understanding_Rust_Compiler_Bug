rust
#![no_std]
#![no_main]
#![feature(lang_items)]
#[link(name="c")]
extern "C" {}

#[lang = "panic_fmt"]
#[no_mangle]
pub fn panic_fmt(_: core::fmt::Arguments, _: &'static str, _: u32, _: u32) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _arg: *const *const u8) -> isize {
    let a: Result<(), usize> = Err(42);
    a.unwrap();
    0
}
