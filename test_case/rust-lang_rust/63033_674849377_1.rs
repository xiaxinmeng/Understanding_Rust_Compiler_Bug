
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
 --> src/main.rs:6:82
  |
6 | async fn something_asynchronous(_a: &'static str, _b: &str, _c: &SomeStruct<'_>) {}
  |                                                                                  ^
  |
note: hidden type `impl std::future::Future` captures lifetime smaller than the function body
 --> src/main.rs:6:82
  |
6 | async fn something_asynchronous(_a: &'static str, _b: &str, _c: &SomeStruct<'_>) {}
  |                                                                                  ^
