rust
#![feature(const_generics)]

pub struct S<const N: usize>([u8; N*0]);
