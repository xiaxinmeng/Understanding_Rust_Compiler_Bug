rust
#![feature(nll)]

use std::ops::*;

#[derive(Copy, Clone)]
struct Foo;

impl AddAssign for Foo {
    fn add_assign(&mut self, _rhs: Foo) {
        panic!()
    }
}

fn bar() {
    let mut a = Foo;
 
    a += a;   
    a.add_assign(a);

}

fn main() {}
