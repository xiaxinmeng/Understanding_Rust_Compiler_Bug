 rust
// src/main.rs
#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate foo;

#[lang = "panic_fmt"]
fn panic_fmt() {}
