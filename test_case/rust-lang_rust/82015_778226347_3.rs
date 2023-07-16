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
        match *self {
            Double { x: ref __self_0_0 } =>
            Double{x: ::core::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
