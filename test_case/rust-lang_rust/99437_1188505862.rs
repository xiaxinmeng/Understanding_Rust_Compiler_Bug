rust
use std::ptr::{self, addr_of_mut};

pub fn test(ptr: *mut [u8]) -> *mut [u8] {
    let layout_size = 24;
    unsafe { ptr::slice_from_raw_parts_mut(ptr.cast::<u8>(), layout_size) }
}
