 rust
use std::marker::PhantomData;
pub struct Invariant<T>(PhantomData<fn(&T)>);
fn send<T:Send>(_:T) {}
fn foo<T:Default>() {
    send(Invariant::<T>(PhantomData));
}
fn main() {
    foo::<i32>();
}
