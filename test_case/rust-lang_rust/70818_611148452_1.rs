
error: future cannot be sent between threads safely
 --> issue-70818.rs:8:17
  |
6 | fn assert_send(_: impl Send) {}
  |    -----------         ---- required by this bound in `assert_send`
7 | fn test<T>(x: T) {
8 |     assert_send(foo(x));
  |                 ^^^^^^ future returned by `foo` is not `Send`
  |
  = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `T`
note: future is not `Send` as this value is used across an await
 --> issue-70818.rs:3:5
  |
2 | async fn foo<T>(x: T) -> T {
  |                 - has type `T` which is not `Send`
3 |     noop().await;
  |     ^^^^^^^^^^^^ await occurs here, with `x` maybe used later
4 |     x
5 | }
  | - `x` is later dropped here
help: consider restricting type parameter `T`
  |
7 | fn test<T: std::marker::Send>(x: T) {
  |          ^^^^^^^^^^^^^^^^^^^
