
#![feature(slice_patterns)]
fn main() { let x: &[i32] = &[]; let &[[_a, ref _b..]..] = x; }
