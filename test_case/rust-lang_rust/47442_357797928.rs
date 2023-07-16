Rust


#![feature(lang_items, start)]
#![no_std]

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

struct Foo {}

impl core::ops::Drop for Foo {
    fn drop(&mut self) {
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let _foo = (Foo {}, Foo {}); // note change here
    return 0;
}
