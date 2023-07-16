 rust
#![no_std]
#![feature(lang_items, globs)]
#![crate_type = "dylib"]

extern crate core;
extern crate collections;
use core::prelude::*;
use collections::string::String;

#[no_mangle]
pub extern "C" fn get_len() -> uint {
    let mut s = String::from_str("Test");
    s.push('_'); // Do enough work to prevent LLVM optimizing it out
    s.len()
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() { }
#[lang = "eh_personality"] extern fn eh_personality() { }
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop { } }
