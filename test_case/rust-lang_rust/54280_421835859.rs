 rust
#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

#[no_mangle]
pub unsafe fn foo(x: &AtomicUsize) {
    x.fetch_add(1, Ordering::SeqCst);
}
