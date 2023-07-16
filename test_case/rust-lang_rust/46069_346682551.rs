Rust
use std::iter::Fuse;

pub struct TupleCombinations<I: Iterator> {
    iter: <Fuse<I> as Iterator>::Item,
}

pub fn equal_combinations_2(
    a: TupleCombinations<std::iter::Cloned<std::slice::Iter<u8>>>
) {
}

fn main() {}
