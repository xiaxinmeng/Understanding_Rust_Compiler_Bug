rust
error: unused variable: `p1`
 --> src/lib.rs:8:39
  |
8 |     fn _f1(#[allow(unused_variables)] p1: u8) {}
  |                                       ^^ help: consider prefixing with an underscore: `_p1`
  |
note: lint level defined here
 --> src/lib.rs:1:9
  |
1 | #![deny(unused_variables)]
  |         ^^^^^^^^^^^^^^^^
