 rust
trait Foo {
    fn foo(&self, &u8);
}

struct Bar;

impl Foo for Bar {
    fn foo(&self, _: &u16) {}
}

fn main() {}
