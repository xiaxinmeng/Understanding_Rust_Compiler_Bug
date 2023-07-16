rust
#![feature(impl_trait_in_bindings)]

trait Foo {
    type Arg;
    type Bar: Fn(&Self::Arg);

    fn bar(&self) -> Self::Bar;
}

struct X;

impl X {
    fn test<F>(this: F)
    where
        F: Foo,
    {
        let function: impl Fn(&F::Arg) = this.bar();
    }
}
