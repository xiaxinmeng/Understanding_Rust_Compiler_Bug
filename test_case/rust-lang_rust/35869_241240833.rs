 rust
trait Foo {
    fn foo(&self, &str);
}

struct Bar;

impl Foo for Bar {
    fn foo(&self, _: &[u8]) {}
}

fn main() {}
