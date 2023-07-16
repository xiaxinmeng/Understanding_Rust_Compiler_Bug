rs
#![feature(const_generics)]
struct X<const L: usize> {
    foo: u8,
    bar: [X<0>;L],
}
