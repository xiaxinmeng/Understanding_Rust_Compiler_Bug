 rust
use std::marker::PhantomData;

trait Foo<T>: Clone {
}

#[derive(Clone)]
pub struct KeyedIgnoring<T> {
    witness: PhantomData<T>,
}

impl<T> Foo<T> for KeyedIgnoring<T> /* where Foo<T> : Clone */ {
    // error: the trait `core::clone::Clone` is not implemented for the type `T`
}

fn main() {
    println!("Hello, world!");
}
