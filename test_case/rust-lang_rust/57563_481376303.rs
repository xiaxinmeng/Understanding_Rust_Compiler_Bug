rust
#![feature(const_vec_new)]

const A: Vec<usize> = Vec::new();

pub const fn return_empty_vec() -> Vec<usize> { Vec::new() }

pub const fn return_vec_via_const_item() -> Vec<usize> { A }
