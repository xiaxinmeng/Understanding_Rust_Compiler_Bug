 rust
use std::marker::PhantomFn;

trait Trait<T>: PhantomFn<Self, T> {}

impl<T> Trait<T> for () where Trait<T>: Eq {}


fn foo<T, U: Trait<T>>() {}


impl PartialEq for Trait<u16> {
    fn eq(&self, _: &Trait<u16>) -> bool { false }
}
impl Eq for Trait<u16> {}


fn main() {
    foo::<u16, ()>();

    // foo::<u8, ()>();
    // error: the trait `core::cmp::Eq` is not implemented for the type `Trait<u8>`
}
