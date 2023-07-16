rust
use std::slice;
use std::usize;

fn main() { unsafe {
    let ptr = Box::into_raw(Box::new(0u8));
    let _x = slice::from_raw_parts(ptr, usize::MAX);
} }
