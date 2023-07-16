rust
#![no_std]
#![feature(lang_items)]

#[lang = "panic_fmt"]
fn foo() {}
#[lang = "eh_personality"]
fn bar() {}

#[cfg(no_std)]
fn main() {}
