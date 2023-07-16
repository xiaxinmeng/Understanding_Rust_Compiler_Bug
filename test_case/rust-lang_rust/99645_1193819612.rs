rust
trait Foo {
    fn foo(&self, a: A) {}
}

impl<'a, F: Fn(A<'a>)> Foo for F {
    fn foo(&self, a: A) {
        self(a);
    }
}
