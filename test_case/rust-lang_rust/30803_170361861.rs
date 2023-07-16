 rust
#![feature(lang_items)]
#![feature(no_core)]

#![crate_type = "lib"]

#![no_std]
#![no_core]

#[lang = "sized"]
trait Sized {}

pub fn foo() {
    for _ in () {
    }
}
