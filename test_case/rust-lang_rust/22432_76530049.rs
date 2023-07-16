 rust
#![crate_type = "lib"]
#![crate_name = "icetest"]

#![no_std]
#![feature(no_std, core)]
extern crate core;

const ZERO: [u32; 8] = [0; 8];

pub fn problematic() {
    let mut _zero = ZERO;
}
