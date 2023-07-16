
error[E0277]: `()` is not a future
  --> src/lib.rs:7:5
   |
7  |     boo().await;
   |     ^^^^^^^^^^^ `()` is not a future
   |
   = help: the trait `Future` is not implemented for `()`
note: required by `poll`
