rust
use std::marker::PhantomData;

struct Foo<'a>(&'a (), PhantomData<fn(&'a ()) -> &'a ()>);

fn covariant<'a>(v: Foo<'static>) -> Foo<'a> {
    v // ERROR mismatched types
}

fn covariant_xd<'a>(_: Foo<'static>) -> Foo<'a> {
    Foo::<'a>::ASSOC
}

impl<'a> Foo<'a> {
    const ASSOC: Foo<'a> = Foo(&(), PhantomData);
}
