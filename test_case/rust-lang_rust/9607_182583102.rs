 rust
use std::marker::PhantomData;

struct Foo<T> {
  marker: PhantomData<T>
}

trait Bar {}

impl<T: Bar> Clone for Foo<T> {
    fn clone(&self) -> Foo<T> { Foo {marker: PhantomData} }
}

#[derive(Clone)]
struct Baz<T> {
    x: Foo<T>
}

fn main() {}
