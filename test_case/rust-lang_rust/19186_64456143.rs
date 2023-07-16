 rust
#![feature(unboxed_closures)]

use std::{iter, slice};

struct Map<A, B, I, F> where I: Iterator<A>, F: FnMut(A) -> B {
    iter: I,
    f: F,
}

fn bytes<'a>(s: &'a str) -> Map<&'a u8, u8, slice::Items<'a, u8>, fn(&u8) -> u8> {
    //~^ error: the trait `core::ops::Fn<(&'a u8,), u8>` is not implemented for the type `fn(&u8) -> u8`
    fn deref(&b: &u8) -> u8 { b }

    Map {
        iter: s.as_bytes().iter(),
        f: deref,
    }
}

fn this_works() -> Map<u8, uint, iter::Range<u8>, fn(u8) -> uint> {
    fn cast(i: u8) -> uint { i as uint }

    Map {
        iter: range(0, 5),
        f: cast,
    }
}

fn main() {}
