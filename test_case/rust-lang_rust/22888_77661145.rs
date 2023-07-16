 rust
use std::io::prelude::*;

trait Foo: Write {
    fn foo(&mut self) {}
}

impl<W: Write> Foo for W {}

fn foo<W: Write+?Sized>(/* HERE */ mut w: &mut W) {
    w.foo();
}

fn main() {
    foo(&mut std::io::sink());
}
