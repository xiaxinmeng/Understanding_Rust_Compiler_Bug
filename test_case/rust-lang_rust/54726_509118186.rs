
error[E0658]: non-builtin inner attributes are unstable
 --> src/main.rs:1:1
  |
1 | #![rustfmt::skip]
  | ^^^^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/54726
  = help: add #![feature(custom_inner_attributes)] to the crate attributes to enable

