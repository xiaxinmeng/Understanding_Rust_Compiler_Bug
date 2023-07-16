
error[E0599]: no method named `poll` found for opaque type `impl std::future::Future` in the current scope
 --> src/lib.rs:8:9
  |
8 |     f().poll(cx);
  |         ^^^^ method not found in `impl std::future::Future`

warning: unused import: `std::future::Future`
 --> src/lib.rs:1:5
  |
1 | use std::future::Future; //would be necessary to call `poll` if the receiver type was correct
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted
