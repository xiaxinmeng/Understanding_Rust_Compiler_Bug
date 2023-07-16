
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
use std::collections::HashMap;
use std::hash::Hash;

fn group_by<I, F, T>(xs: &mut I, f: F) -> HashMap<T, Vec<&<I>::Item>> where
 I: Iterator, F: Fn(&<I>::Item) -> T, T: Eq +
 Hash {
          let mut result = <HashMap>::new();
          {
              let _t =
                  match ::std::iter::IntoIterator::into_iter(xs) {
                      mut iter =>
                      loop  {
                          let mut __next;
                          match ::std::iter::Iterator::next(&mut iter) {
                              ::std::option::Option::Some(val) =>
                              __next = val,
                              ::std::option::Option::None => break ,
                          }
                          let ref x = __next;
                          {
                              let key = f(x);
                              result.entry(key).or_insert(<Vec>::new()).push(x);
                          }
                      },
                  };
              _t
          };
          result
      }

fn main() { }
