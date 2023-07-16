
error: unreachable expression
 --> src/main.rs:6:33
  |
6 |     let bar = foo().map_err(|e| e.to_string());
  |                                 ^^^^^^^^^^^^^
  |
note: lint level defined here
 --> src/main.rs:1:9
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: #[deny(unreachable_code)] implied by #[deny(warnings)]

error: aborting due to previous error
