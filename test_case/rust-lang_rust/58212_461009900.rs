rust
use std::io::{empty, Read};

trait Foo {
    fn foo();
}

impl Foo for [u8; 10] {
    fn foo() {
        let mut result: Self = [0; 10];
        empty().read(&mut result).unwrap();
    }
}
