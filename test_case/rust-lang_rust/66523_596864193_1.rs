
error[E0277]: the trait bound `(): std::future::Future` is not satisfied
 --> src/lib.rs:1:13
  |
1 | fn foo() -> impl std::future::Future<Output = i32> {
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `()`
2 |     unimplemented!();
  |     ----------------- consider removing this semicolon
  |
  = note: the return type of a function must have a statically known size
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
