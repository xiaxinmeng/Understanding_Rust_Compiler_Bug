rust
#![feature(layout_for_ptr)]

use std::{mem, ptr};

const PREFIX: usize = 4;

#[repr(C)]
struct DST {
    align: [u64; 0],
    prefix: [u8; PREFIX],
    slice: [u8],
}

fn main() {
    for len in 0..32 {
        let ptr = ptr::slice_from_raw_parts(ptr::null::<()>(), len);
        let ptr = ptr as *const DST;
        let offset = unsafe {
            ptr::addr_of!((*ptr).slice)
                .cast::<u8>()
                .offset_from(ptr.cast())
        };
        print!("{offset} ");
    }
    println!();
}
