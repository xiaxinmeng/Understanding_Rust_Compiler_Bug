 Rust
use std::ops::Range;

pub trait Iter {
    type Type: Iterator;
}

pub trait T {
    fn f<'a>(&'a self) -> <&'a Self as Iter>::Type where &'a Self: Iter;
}

pub struct S;

impl<'a> Iter for &'a S {
    type Type = Range<usize>;
} 

impl T for S {
    fn f<'a>(&'a self) -> <&'a Self as Iter>::Type
            // need some (holding) bound on 'a to make it early-bound
            where &'a (): Sized {
        0usize..10
    }
}

fn main() {}
