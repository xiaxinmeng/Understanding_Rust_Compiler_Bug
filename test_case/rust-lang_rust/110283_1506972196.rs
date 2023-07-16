rust
#![feature(lang_items)]
#![feature(no_core)]
#![no_core]

#[lang = "sized"]
trait Foo {}

pub unsafe fn foo(x: *const i32) -> &'static i32 { unsafe { &*x } }
