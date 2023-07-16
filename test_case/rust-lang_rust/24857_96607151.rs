 rust
use std::marker::PhantomData;

struct Foo<T = ()>(PhantomData<T>);

fn f<S>(_: Foo<S>) {}

fn main() {
    f(Foo(PhantomData));
}
