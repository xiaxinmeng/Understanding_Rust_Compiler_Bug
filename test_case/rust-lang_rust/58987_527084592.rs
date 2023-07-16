rust
#![feature(const_generics)]

struct NibblePack<const N: usize> ([u8; N / 1]);
