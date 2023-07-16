rust
#![allow(unused)]

type A = [u8; { struct S; 1 }]; // OK since 1.0

fn main() {}
