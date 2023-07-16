rust
impl Trait for &Thingy {
    type FooFn<'a> = impl FnOnce() + 'a where Self: 'a;
    fn foo(&mut self) -> Self::FooFn<'_> {
        self.foo_inner()
    }
}

impl Thingy {
    fn foo_inner(&self) -> impl FnOnce() + '_ {
        move || {
            println!("{}", self.0);
        }
    }
}
