 rust
#![no_std]
#![feature(no_std,core,lang_items)]

extern crate core;

#[lang="start"]
fn main() {
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }
