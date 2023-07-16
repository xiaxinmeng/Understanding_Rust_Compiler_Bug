 rust
#![feature(lang_items, no_std)]
#![no_std]

extern crate core;

use core::ops::Drop;

#[lang = "panic_fmt"]
fn panic_fmt() {}

#[lang = "stack_exhausted"]
fn stack_exhausted() {}

#[lang = "eh_personality"]
fn eh_personality() {}

impl<T: Drop> Drop for T {
    fn drop(&mut self) {}
}

fn main() {}
