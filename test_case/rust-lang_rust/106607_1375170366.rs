rust
use std::ops::Deref;

trait Foo {
    fn foo(&self) -> &Self::Target where Self: Deref { &**self }
}

trait Bar {
    fn foo(&self) {}
}

fn test(i: impl Foo + Bar) {
    i.foo();
}
