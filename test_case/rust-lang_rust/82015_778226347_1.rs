rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
struct Double {
    x: Iterator<Item = i32>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Double {
    #[inline]
    fn clone(&self) -> Double {
        {
            let _: ::core::clone::AssertParamIsClone<Iterator<Item = i32>>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Double { }
