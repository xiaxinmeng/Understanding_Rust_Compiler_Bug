
error: unused variable: `a`
 --> src/main.rs:5:9
  |
5 |     let a = 5; // Still gets an error
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
  |
note: the lint level is defined here
 --> src/main.rs:2:8
  |
2 | #[deny(warnings)]
  |        ^^^^^^^^
  = note: `#[deny(unused_variables)]` implied by `#[deny(warnings)]`
