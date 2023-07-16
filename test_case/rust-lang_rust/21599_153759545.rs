
#![no_std]
#![feature(box_syntax)]
#![feature(unique)]
#![feature(no_std)]
#![feature(lang_items)]

use core::ptr::Unique;

#[lang="owned_box"]
pub struct Box<T>(Unique<T>);

#[lang="start"]
fn main() {
    let mut test:[isize;1] = [0;1];
    let a = box 5;
    test[*a] = 0;
}

#[lang = "exchange_malloc"] extern fn exchange_malloc() {}
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }

