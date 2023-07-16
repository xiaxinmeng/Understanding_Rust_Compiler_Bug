 rust
#![no_implicit_prelude]

use std::boxed::Box;
use std::marker::Sized;
use std::option::Option;

trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl<'a, T> Iterator<T> for &'a mut (Iterator<T> + 'a) {
    fn next(&mut self) -> Option<T> {
        loop {}
    }
}

trait IteratorExt<T>: Iterator<T> + Sized {
    fn last(self) -> Option<T> {
        loop {}
    }
}

impl<I, T> IteratorExt<T> for I where I: Iterator<T> {}

fn test(mut it: Box<Iterator<u8> + 'static>) {
    it.last();  // <-- it works
}

fn main() {}
