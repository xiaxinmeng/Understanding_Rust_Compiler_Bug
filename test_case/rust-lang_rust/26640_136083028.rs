 rust
#![feature(no_std, core)]
#![no_std]
extern crate core;

pub fn burger() {}
extern {
    pub fn fries();
}
pub extern fn drink() {}
