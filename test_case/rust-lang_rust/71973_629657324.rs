rust
#![feature(const_generics)]
#![allow(incomplete_features)]

pub const fn sof<T>() -> usize {
    10
}

fn test<T>() {
    let _: [u8; sof::<T>()];
}

fn main() {}
