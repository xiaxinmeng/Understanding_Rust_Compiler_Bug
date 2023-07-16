rust
#![feature(const_generics)]
struct S<const N: u8>;
impl S<256> { fn foo() {} }
