
#![feature(simd)]
#![allow(experimental)]

use std::simd::i32x4;

fn main() {
    let x = i32x4(1, 2, 3, 4);
    let y = i32x4(7, 2, 3, 4);
    let _ = x == y;
    println!("{}", x == y);
}
