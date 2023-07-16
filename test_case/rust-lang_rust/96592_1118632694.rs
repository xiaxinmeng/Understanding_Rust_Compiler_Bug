rust
#![feature(inline_const)]
#![allow(incomplete_features)]

fn test<const N: usize>() {
}

fn main() {
    let _ = test::<{const {1}}>();
}
