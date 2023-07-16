
error: in expressions, `_` can only be used on the left-hand side of an assignment
 --> src/lib.rs:3:13
  |
3 |     s = [0; _];
  |             ^ `_` not allowed here

error[E0658]: using `_` for array lengths is unstable
 --> src/lib.rs:3:13
  |
3 |     s = [0; _];
  |             ^
  |
  = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
  = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
