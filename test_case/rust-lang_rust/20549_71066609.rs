
#![no_std]
#![feature(box_syntax)]
#![feature(lang_items)]

#[allow(unstable)]
extern crate core;

use core::marker::Copy;

fn main() {
    let foo = box 5;
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }
