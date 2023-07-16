
#![feature(self_struct_ctor)]
#![allow(dead_code)]

enum Foo {}

impl Foo {
    fn bar() {
        Self;
    }
}

fn main() {}
