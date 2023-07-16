
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
 --> src/lib.rs:4:54
  |
4 |     async fn wat(&self, _: &'static str, _: Bar<'_>) {}
  |                                                      ^
  |
note: hidden type `impl std::future::Future` captures the scope of call-site for function at 4:54
 --> src/lib.rs:4:54
  |
4 |     async fn wat(&self, _: &'static str, _: Bar<'_>) {}
  |                                                      ^^

error: aborting due to previous error
