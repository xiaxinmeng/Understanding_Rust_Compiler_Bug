 rust
use std::marker::PhantomFn;

trait Trait<T>: PhantomFn<Self, T> {}

impl<T> Trait<T> for () where Trait<T>: Eq {}

fn main() {}
