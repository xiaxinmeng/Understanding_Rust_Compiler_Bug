
#![feature(decl_macro)]
#![feature(underscore_imports)]
#![allow(unused)]

struct S;

mod m {
    pub trait Tr {
        fn f(&self) {}
    }
    impl Tr for ::S {}
}

macro_rules! m {
  () => {
    use m::Tr as _;
  }
}

m!();

fn main() {
    S.f(); // ERROR no method named `f` found for type `S` in the current scope
}
