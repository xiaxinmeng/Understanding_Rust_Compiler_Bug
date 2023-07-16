
#![feature(const_slice_len)]
#![allow(unused_variables)]

fn main() {
    let a: [u8; 3] = [10, 20, 30];
    let b: [u8; a.len()] = Default::default();
    // error[E0435]: attempt to use a non-constant value in a constant
}
