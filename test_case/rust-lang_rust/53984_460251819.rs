rust
trait Foo {
    type Arg;
    type Assoc: Fn(Self::Arg);

    fn assoc() -> Self::Assoc;
    fn baz(_: Self::Assoc);
}

fn test<F: Foo<Arg = usize>>(this: F) {
    F::baz(F::assoc());
}
