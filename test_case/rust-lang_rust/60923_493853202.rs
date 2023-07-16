rust
#![feature(const_generics)]

#[derive(PartialEq)]
struct S<T, const N: usize>([T; N]);

fn main() {}
