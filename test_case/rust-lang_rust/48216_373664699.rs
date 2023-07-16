rust
#![feature(decl_macro)]
#![allow(unused)]

struct S;

mod m {
    pub trait Tr {
        fn f(&self) {}
    }
    impl Tr for ::S {}
}

pub macro m() {
    use m::Tr;
}

m!();

fn main() {
    S.f(); // ERROR no method named `f` found for type `S` in the current scope
}
