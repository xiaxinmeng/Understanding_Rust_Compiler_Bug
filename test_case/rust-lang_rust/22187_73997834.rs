 rust
#![feature(no_std)]
#![feature(lang_items)]
#![crate_type="lib"]
#![no_std]

#[lang="sized"]
pub trait Sized { }

trait T {
    type Ty;
}
