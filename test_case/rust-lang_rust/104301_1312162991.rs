rust
#![no_std]

use core::cell::UnsafeCell;

pub fn foo() -> UnsafeCell<()> {
    UnsafeCell::new(())
}
