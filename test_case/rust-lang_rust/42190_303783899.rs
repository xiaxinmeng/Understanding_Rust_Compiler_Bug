 rust
#![no_std]

#[cfg(no_std)]
use core::mem;

fn foo() {
    let x: u32 = unsafe { mem::uninitialized() };
}
