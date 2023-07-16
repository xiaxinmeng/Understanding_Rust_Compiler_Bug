rust
#![allow(dead_code)]

use std::cell::Cell;
use std::marker::PhantomData;

struct Foo<F, A, R>
    where F: Fn(A) -> R
{
    f: F,
    d: PhantomData<fn(A)>,
}

macro_rules! foo_ty {
    ($a:ty, $b:ty) => {
        Foo<fn($a) -> $b, $a, $b>
    }
}

trait Extract {
    type Out;
    fn create(self) -> Self::Out;
}

impl<F, A, R> Extract for Foo<F, A, R>
    where F: Fn(A) -> R
{
    type Out = R;

    fn create(self) -> Self::Out { panic!() }
}

fn foo<'a, 'b, 'small, 'big>(a: foo_ty!(&'a (), &'a ()),
                             b: foo_ty!(&'b (), &'b ()))
                             -> Cell<&'small ()>
    where 'a: 'small, 'b: 'small, 'big: 'a, 'big: 'b,
{
    let x = match 22 { 0 => a, 1 => a, 2 => a, _ => b };
    let y = Cell::new(Extract::create(x));
    y
}

fn main() {
}
