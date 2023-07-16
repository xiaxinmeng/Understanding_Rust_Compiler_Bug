rust
use std::marker::PhantomData;

fn does_not_compile<'b>(y: PhantomData<&'b ()>) -> Box<for<'s: 'b> Fn() -> PhantomData<&'s ()>> {
    Box::new(move ||y)
}
fn does_compile<'b, 's:'b>(y: PhantomData<&'b ()>) -> Box<Fn() -> PhantomData<&'s ()>> {
    Box::new(move ||y)
}
fn main() {}
