 rust
#![no_std]

use core::sync::atomic::{Ordering, AtomicUsize};

#[no_mangle]
pub fn foo(x: &AtomicUsize) -> usize {
    x.load(Ordering::SeqCst)
}
