rust
// Uncomment to make error work:
//extern crate repro;

pub struct Bar;

impl repro::Foo for Bar {
    fn foo() {}

    // This compiles!
    fn extra() {}
}

fn main() {}
