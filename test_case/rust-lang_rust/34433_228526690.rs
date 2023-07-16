
use std::usize;

fn main() {
    let iter = (0..usize::max_value()).chain(0..usize::max_value());
    println!("{:?}", iter.size_hint());
}
