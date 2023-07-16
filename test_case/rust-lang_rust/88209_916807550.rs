
error[E0658]: destructuring assignments are unstable
 --> src/main.rs:3:5
  |
3 |     _;
  |     ^
  |
  = note: see issue #71126 <https://github.com/rust-lang/rust/issues/71126> for more information

error: in expressions, `_` can only be used on the left-hand side of an assignment
 --> src/main.rs:3:5
  |
3 |     _;
  |     ^ `_` not allowed here
