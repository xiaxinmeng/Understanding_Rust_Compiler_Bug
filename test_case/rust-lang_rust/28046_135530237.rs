 rust
use std::ops::Range;

pub trait Iter {
    type Type: Iterator;
}

pub struct S;

impl<'a> Iter for &'a S {
    type Type = Range<usize>;
} 

impl S {
    fn f<'a>(&'a self) -> Range<usize> where &'a S: Iter {
        let x: <&'static S as Iter>::Type = 0usize..10;
        x
    }
}

fn main() {}
