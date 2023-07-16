rust
#![allow(incomplete_features)]
#![feature(adt_const_params)]
use std::slice::from_raw_parts;

struct AsPtr<const SLICE: &'static [u8]>;
impl<const SLICE: &'static [u8]> AsPtr<SLICE> {
    const PTR: *const u8 = SLICE.as_ptr();
}

fn main() {
    let ptr = AsPtr::<{ unsafe { from_raw_parts(1 as *const u8, 0) } }>::PTR;
    println!("{ptr:p}");
}
