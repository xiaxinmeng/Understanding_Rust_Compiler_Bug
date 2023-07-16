console
error: expected one of `.`, `?`, `]`, or an operator, found `:`
 --> src/main.rs:2:25
  |
2 |     let a = &[1, 2, 3][1:2];
  |                         ^ expected one of `.`, `?`, `]`, or an operator
  |
  = note: type ascription syntax has been removed, [see issue #101728 <https://github.com/rust-lang/rust/issues/101728>](https://github.com/rust-lang/rust/issues/101728)
help: maybe write a path separator here
  |
2 |     let a = &[1, 2, 3][1::2];
