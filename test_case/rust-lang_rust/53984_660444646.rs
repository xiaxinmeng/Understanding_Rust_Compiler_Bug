rust
trait Foo {
    type Arg;
    type Assoc: Fn(Self::Arg);

    fn assoc() -> Self::Assoc;
    fn baz(_: Self::Assoc);
}

fn test<C: Fn(usize), F: Foo<Arg = usize, Assoc = C>>(this: F) {
    F::baz(F::assoc());
}
