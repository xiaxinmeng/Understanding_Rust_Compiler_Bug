rust
use std::convert::Infallible;
use std::marker::PhantomData;

enum Foo<T> {
    _Bar(Infallible, PhantomData<T>),
}
