rust
#![feature(unsize, dispatch_from_dyn)]

use std::marker::Unsize;
use std::ops::DispatchFromDyn;

pub struct Foo<'a, T: ?Sized> {
    _inner: &'a &'a T, // removing outer borrow here is enough to suppress the ICE
}

impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Foo<'a, U>> for Foo<'a, T> {}
