 rust
#![crate_type = "lib"]
#![feature(i128_type)]
#![feature(lang_items)]
#![feature(no_core)]
#![no_core]

fn foo(x: i128) {}

#[lang = "copy"]
trait Copy {}

#[lang = "sized"]
trait Sized {}
