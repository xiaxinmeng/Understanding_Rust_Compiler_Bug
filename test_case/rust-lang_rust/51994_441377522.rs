rust
#![feature(self_struct_ctor)]
#![allow(dead_code)]

enum Foo {}

impl Foo {
    fn bar() {
        Self(1u8);
    }
}

fn main() {}
