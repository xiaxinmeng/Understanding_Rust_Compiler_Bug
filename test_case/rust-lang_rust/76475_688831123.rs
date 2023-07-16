rust
use std::mem::size_of;

fn main() {
    type Int = u64;
    let mut k: Int = 1_000_000_000;
    const N: usize = size_of::<Int>() * 8;
}
