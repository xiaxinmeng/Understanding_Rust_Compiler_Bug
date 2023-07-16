rust
error: higher-ranked lifetime error
 --> src/lib.rs:5:5
  |
5 |     assert_send(bar());
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: could not prove `impl futures::Future<Output = Vec<i32>>: std::marker::Send`
