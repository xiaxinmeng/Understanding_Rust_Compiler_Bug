 rust
trait Foo {
    fn foo(&self) { fail!() }
}

impl<'a, T> Foo for &'a T where T: Clone {}

trait UsableFoo {
    fn foo(&self) {}
}

struct Bar;

impl UsableFoo for Bar {}

fn main() {
    let b = &&Bar;
    b.foo();
}
