
#![allow(dead_code)]
#![feature(exclusive_range_pattern)]

fn foo(x: u8) -> bool {
    match x {
        0 .. 101 => true,
        100 ... 255 => false,
    }
}

fn main() {}
