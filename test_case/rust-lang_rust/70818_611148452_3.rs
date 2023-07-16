
error: future cannot be sent between threads safely
...
  = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `T`
note: future is not `Send` as this value is used across an await
 --> issue-70818.rs:4:5
  |
2 | async fn foo<T>(x: T) {
  |                 - has type `T` which is not `Send`
3 |     std::mem::drop(x);
4 |     noop().await;
  |     ^^^^^^^^^^^^ await occurs here, with `x` maybe used later
5 | }
  | - `x` is later dropped here
...
