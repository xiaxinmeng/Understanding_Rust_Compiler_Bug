rust
#![feature(exact_size_is_empty, range_is_empty)]

use std::iter::ExactSizeIterator;

fn main() {
    println!("{:?}", (0..0).is_empty());
}
