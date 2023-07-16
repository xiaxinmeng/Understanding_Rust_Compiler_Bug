
error[E0658]: destructuring assignments are unstable
 --> src/main.rs:3:17
  |
3 |     if y < Some(_) {}
  |                 ^
  |
  = note: see issue #71126 <https://github.com/rust-lang/rust/issues/71126> for more information
  = help: add `#![feature(destructuring_assignment)]` to the crate attributes to enable

error: in expressions, `_` can only be used on the left-hand side of an assignment
 --> src/main.rs:3:17
  |
3 |     if y < Some(_) {}
  |                 ^ `_` not allowed here

error: aborting due to 2 previous errors
