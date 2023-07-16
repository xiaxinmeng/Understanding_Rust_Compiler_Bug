rust
#![feature(trait_alias)]

mod m {
    pub trait A { fn foo() {} }
    pub trait B { fn bar() {} }
    
    impl A for u8 {}
    impl B for u8 {}
}

trait C = m::A + m::B;

fn main() {
    u8::foo(); // OK
}
