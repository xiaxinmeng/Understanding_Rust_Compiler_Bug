
error[E0658]: associated type bounds are unstable
 --> src/lib.rs:5:19
  |
5 | fn foo<T: FromStr<Err: Debug>>(a: T) {}
  |                   ^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/52662
  = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
