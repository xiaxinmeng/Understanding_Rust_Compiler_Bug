rust
#![feature(trait_alias)]

mod alpha {
    pub trait A { fn foo() {} }
    pub trait B { fn foo() {} }
    // pub trait C = A + B;
    
    impl A for u8 {}
    impl B for u8 {}
}

// use alpha::C;
use alpha::*;

fn main() {
    u8::foo(); // ERROR multiple applicable items in scope
}
