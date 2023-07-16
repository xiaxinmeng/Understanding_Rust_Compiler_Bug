rs
#![feature(dyn_star)]
#![allow(incomplete_features)]

use std::fmt::Debug;

#[repr(C)]
#[derive(Debug)]
struct AlignedUsize(usize);

fn main() {
    let _x = AlignedUsize(12) as dyn* Debug;
}
