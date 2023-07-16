 rust
use std::marker::PhantomFn;
trait Dummy<T> : PhantomFn<Self, T> { }
fn main() {}
