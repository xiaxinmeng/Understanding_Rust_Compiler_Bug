
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[deprecated]
fn _foo() {
    ::std::fmt::Arguments::new_v1({
                                      static __STATIC_FMTSTR: &'static [&'static str] = &["bla"];
                                      __STATIC_FMTSTR
                                  },
                                  &match () {
                                      () => [],
                                  });
}

fn main() {}
