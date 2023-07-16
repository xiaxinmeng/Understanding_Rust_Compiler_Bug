rust
#![feature(nll)]

use std::num::Wrapping;

fn main() {
    let mut x = Wrapping(1234u32);
    x ^= x >> 2;
    
    println!("{}", x);
}
