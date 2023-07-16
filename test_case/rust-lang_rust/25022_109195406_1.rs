 rust
#![feature(no_std)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
struct Foo<T>(pub T);
#[automatically_derived]
impl <T: ::std::clone::Clone> ::std::clone::Clone for Foo<T> where
 T: ::std::clone::Clone {
    #[inline]
    fn clone(&self) -> Foo<T> {
        match *self {
            Foo(ref __self_0_0) =>
            Foo(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}

fn main() { }
