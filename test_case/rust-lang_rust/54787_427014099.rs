rust
#![feature(nll)]
#![allow(unreachable_code)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;

fn main() {
              {
                  let _result =
                      match ::std::iter::IntoIterator::into_iter({
                                                                     return ();
                                                                     ::std::ops::Range{start:
                                                                                           0,
                                                                                       end:
                                                                                           3,}
                                                                 }) {
                          mut iter =>
                          loop  {
                              let mut __next;
                              match ::std::iter::Iterator::next(&mut iter) {
                                  ::std::option::Option::Some(val) =>
                                  __next = val,
                                  ::std::option::Option::None => break ,
                              }
                              let _ = __next;
                              { }
                          },
                      };
                  _result
              }
          }
