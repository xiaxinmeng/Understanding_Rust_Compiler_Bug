 Rust
#![feature(const_fn, integer_atomics)]

use std::cell::UnsafeCell;
use std::sync::atomic::AtomicU64;

pub struct SyncCell<T>(UnsafeCell<T>);
unsafe impl<T: Sync> Sync for SyncCell<T> {}

static XYZ: SyncCell<[u64; 1024*512]> =
    SyncCell(UnsafeCell::new([0; 1024*512]));

pub fn get_xyz() -> &'static [AtomicU64; 1024*512] {
    unsafe {&*(XYZ.0.get() as *mut _)}
}

fn main() {}
