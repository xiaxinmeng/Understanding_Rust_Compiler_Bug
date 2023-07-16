 rust
// file: app/src/main.rs
// Cargo.toml
// [dependencies]
// bar = { path = "../bar" }

#![feature(lang_items)]
#![feature(start)]
#![no_std]

extern crate bar;

use core::ptr;

#[lang = "eh_personality"]
fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() {}

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    let x = bar::bar();
    // prevent LLVM from optimizing the call to `bar` away
    unsafe {
        ptr::read_volatile(&x);
    }
    0
}

#[link(name = "c")]
extern "C" {}
