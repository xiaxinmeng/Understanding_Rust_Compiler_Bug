

error[E0277]: `()` is not a future
  --> src/lib.rs:7:5
   |
7  |     boo().await;
   |     ^^^^^^^^^^^ `()` is not a future
   |
   = note: `()` cannot be `await`ed
help: consider removing the yield point
   |
7  -     boo().await;
7  +     boo();
   |
help: otherwise, consider making `boo` return a `Future` by turning it into an async function
  |
3 | async fn boo() {}
  | +++++
