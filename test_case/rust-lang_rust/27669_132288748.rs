 rust
$ rustc -Z unstable-options --pretty=expanded src/lib.rs
#![feature(no_std)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate term;

fn foo() {
    let mut t = term::stdout().unwrap();
    t.write_fmt(::std::fmt::Arguments::new_v1({
                                                  static __STATIC_FMTSTR:
                                                         &'static [&'static str]
                                                         =
                                                      &["foo\n"];
                                                  __STATIC_FMTSTR
                                              },
                                              &match () {
                                                   () => [],
                                               })).unwrap();
}
