 rust
trait A {}
trait B { fn foo(&self); }

impl<T: A> B for T {
    fn foo(&self) {}
}

fn foo(a: &A) {
    a.foo();
}

fn main() {}
