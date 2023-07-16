rust
use std::marker::PhantomData;

pub struct Invariant<T>(T, PhantomData<*mut T>);
pub struct Foo<T>(T, [u8; 64]);

pub fn abc<'a>(x: &Foo<Invariant<Box<for<'b> fn(&'b u8, &'b u8)>>>)
    -> &Foo<Invariant<Box<for<'b, 'c> fn(&'b u8, &'c u8)>>> { x }

fn main() {
    abc(&Foo(Invariant(Box::new(|_x, _y| ()), PhantomData), [0; 64]));
}
