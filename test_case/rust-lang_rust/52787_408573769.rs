 rust
#![allow(warnings)]
#![crate_type = "lib"]
#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

#[no_mangle]
pub fn foo(x: &AtomicUsize, y: usize) -> usize {
    x.swap(y, Ordering::SeqCst)
}
