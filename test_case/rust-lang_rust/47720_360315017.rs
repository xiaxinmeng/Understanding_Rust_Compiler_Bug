rust
use std::marker::PhantomData;

struct AssertSync<T: Sync>(PhantomData<T>);

pub struct Foo {
    bar: *const Bar,
    phantom: PhantomData<Bar>,
}

pub struct Bar {
    foo: *const Foo,
    phantom: PhantomData<Foo>,
}

fn main() {
    let _: AssertSync<Foo> = unimplemented!();
}
