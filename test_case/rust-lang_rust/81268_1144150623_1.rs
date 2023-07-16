
error[E0658]: using `_` for array lengths is unstable
 --> src/main.rs:2:27
  |
2 |     let _: [u16; 1] = [0; _];
  |                           ^
  |
  = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
  = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
