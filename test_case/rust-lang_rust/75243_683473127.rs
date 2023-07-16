rust
error[E0658]: use of unstable library feature 'array_map'
  --> lambda-twist/tests/consensus.rs:25:6
   |
25 |     .map(|&p| Point3::from(p));
   |      ^^^
   |
   = note: see issue #75243 <https://github.com/rust-lang/rust/issues/75243> for more information
   = help: add `#![feature(array_map)]` to the crate attributes to enable
