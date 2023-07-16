rust
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate static_assertions;

fn main() { }

fn test<T>() {
    #[allow(unknown_lints, eq_op)]
    let _ =
        [();
            0 -
                !({
                      const B: bool = ::std::mem::size_of::<Box<T>>();
                      B
                  }) as usize];
