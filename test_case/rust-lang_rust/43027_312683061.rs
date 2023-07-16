rust
#![feature(reverse_cmp_key)]

use std::cmp::Reverse;

fn main() {
    let later = Reverse(0u32);
    let _ = later.0;
}
