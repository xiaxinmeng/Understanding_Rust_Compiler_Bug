
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/lib.rs:8:26
  |
8 |   fn needs_sized() where Self: Sized {}
  |                          ^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: within `Unsized`, the trait `Sized` is not implemented for `[u8]`
note: required because it appears within the type `Unsized`
 --> src/lib.rs:5:8
  |
5 | struct Unsized([u8]);
  |        ^^^^^^^
  = help: see issue #48214
  = help: [add `#![feature(trivial_bounds)]`](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#) to the crate attributes to enable
