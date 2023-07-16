
Standard Error

   Compiling playground v0.0.1 (/playground)
error[E0277]: `T` cannot be sent between threads safely
 --> src/lib.rs:8:17
  |
1 | async fn foo<T>(x: T) -> T {
  |                          - within this `impl std::future::Future`
...
5 | fn assert_send(_: impl Send) {}
  |    -----------         ---- required by this bound in `assert_send`
...
8 |     assert_send(foo(x));
  |                 ^^^^^^ `T` cannot be sent between threads safely
  |
  = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `T`
  = note: required because it appears within the type `[static generator@src/lib.rs:1:28: 3:2 x:T {}]`
  = note: required because it appears within the type `std::future::from_generator::GenFuture<[static generator@src/lib.rs:1:28: 3:2 x:T {}]>`
  = note: required because it appears within the type `impl std::future::Future`
  = note: required because it appears within the type `impl std::future::Future`
help: consider restricting type parameter `T`
  |
7 | fn test<T: std::marker::Send>(x: T) {
  |          ^^^^^^^^^^^^^^^^^^^
