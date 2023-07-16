rust
pub trait Trait {
    type FooFn<'a>:  FnOnce() + 'a where Self: 'a;

    fn foo(&mut self) -> Self::FooFn<'_>;
}

struct Thingy;

impl<'b> Trait for &'b Thingy {
    type FooFn<'a> = impl FnOnce() + 'a where Self: 'a;
    fn foo<'c>(&'c mut self) -> Self::FooFn<'c> {
        move || {
            let _ = self;
        }
    }
}
