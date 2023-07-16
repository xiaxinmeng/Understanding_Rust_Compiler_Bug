 rust
#![feature(lang_items, no_std, core)]
#![no_std]
extern crate core;

#[lang = "stack_exhausted"]
extern {}
#[lang = "eh_personality"]
extern {}
#[lang = "panic_fmt"]
extern {}
#[lang = "start"]
fn main() {}
