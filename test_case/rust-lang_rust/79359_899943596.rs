rust
trait Foo {
    fn foo(&self) -> String;
}

impl<F> Foo for F where F: Fn() -> String {
    fn foo(&self) -> String {
        self()
    }
}

fn foo(foo: impl Foo) -> String {
    foo.foo()
}

fn bar(x: Box<dyn Foo>) {
    foo(x);
}
