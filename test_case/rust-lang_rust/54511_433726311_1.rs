 rust
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(no_core)]
#![no_core]

extern "rust-intrinsic" {
    fn atomic_singlethreadfence_acq();
}

pub unsafe fn foo(x: *mut u16) -> u16 {
    atomic_singlethreadfence_acq();
    *x
}

#[lang = "copy"]
trait Copy {}

impl Copy for u16 {}

#[lang = "freeze"]
trait Freeze {}

#[lang = "sized"]
trait Sized {}
