rust
#![feature(const_generics)]

struct X;

struct S<const X: u8>;

impl<const X: u8> S<X> {} // error: wrong number of const arguments: expected 1, found 0
