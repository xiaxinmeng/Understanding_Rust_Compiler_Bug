rust
#![feature(generic_associated_types)]

use std::slice::Iter;

pub trait A {
    type Y<'a> where Self: 'a;
    type Z<'a>: Iterator<Item = Self::Y<'a>> where Self: 'a;

    fn iter(&self) -> Self::Z<'_>;
}

pub trait B: for<'a> A<Z<'a> = Iter<'a, <Self as A>::Y<'a>>> {}
