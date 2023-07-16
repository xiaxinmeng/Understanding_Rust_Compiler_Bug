rust
pub trait Foo: Sync {
    fn run<'a, 'b: 'a, T: Sync>(&'a self, _: &'b T) -> impl Future<Output = ()> + 'a;
}

pub trait FooExt: Foo {
    fn run_via<'a, 'b: 'a, T: Sync>(&'a self, t: &'b T) -> impl Future<Output = ()> + 'a {
        async move {
            self.run(t).await; // compiles now!
        }
    }
}
