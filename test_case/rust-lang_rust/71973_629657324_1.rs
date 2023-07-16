rust
// run-pass
#![feature(const_generics)]
#![allow(incomplete_features)]

pub fn test<T>() {
    let _: [u8; std::mem::size_of::<T>()];
}

fn main() {}
