rust
#![cfg_attr(not(std), no_std)]

#[cfg(not(std))]
use core::mem;
#[cfg(std)]
use std::mem;

fn main() {
    let x: u32 = unsafe { mem::uninitialized() };
}
