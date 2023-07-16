rust
#![deny(unused_variables)]

struct Foo;

fn _f0(#[allow(unused_variables)] p0: u8) {}

impl Foo {
    fn _f1(#[allow(unused_variables)] p1: u8) {}
    
    fn _f2(&self, #[allow(unused_variables)] p2: u8) {}
}
