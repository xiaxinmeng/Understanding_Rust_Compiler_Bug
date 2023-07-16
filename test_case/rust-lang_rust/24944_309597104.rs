rust
trait Foo {
    fn foo(&self);
}

struct Bar<T>(T);

impl<T> Bar<T> where Self: Foo {}

fn main() {}
