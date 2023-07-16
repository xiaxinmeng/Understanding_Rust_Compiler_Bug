rust
#![feature(lang_items, naked_functions, no_core)]
#![crate_type = "lib"]
#![no_core]

#[lang = "sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

#[naked]
unsafe fn _f() {}
