rust
#![allow(const_err)]

const CONST_RAW: *const Vec<i32> = &Vec::new() as *const _;

fn main() {}
