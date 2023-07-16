
error[E0658]: const generics are unstable
 --> src/lib.rs:3:16
  |
3 | struct S<const I: usize>;
  |                ^
  |
  = note: see issue #74878 <https://github.com/rust-lang/rust/issues/74878> for more information
  = help: add `#![feature(min_const_generics)]` to the crate attributes to enable
