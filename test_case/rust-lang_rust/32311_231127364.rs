
#![feature(range_contains)]
use std::ops::Range;
use std::num::Wrapping;
use std::u32;

fn main() {
    let r1: Range<Wrapping<u32>> = Range{ start: Wrapping((u32::MAX - 100)), end: Wrapping(100) };
    println!("{}", r1.contains(Wrapping(u32::MAX)));
}
