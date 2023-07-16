
error[E0515]: cannot return reference to local data `arena`
 --> src/lib.rs:5:18
  |
5 |     async move { &arena }.await;
  |                  ^^^^^^ returns a reference to data owned by the current function

error: aborting due to previous error
