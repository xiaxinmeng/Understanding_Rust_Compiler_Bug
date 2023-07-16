
error[E0658]: arbitrary expressions in key-value attributes are unstable
 --> src/lib.rs:4:11
  |
4 |     doc = include_str!("../README.md")
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
  = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable
