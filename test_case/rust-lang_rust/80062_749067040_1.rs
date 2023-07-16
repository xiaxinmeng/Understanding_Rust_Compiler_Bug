
error[E0277]: the size for values of type `T` cannot be known at compilation time
 --> src/lib.rs:3:23
  |
1 | fn sof<T>() -> T { todo!() }
  |        - required by this bound in `sof`
2 | fn test<T>() {
  |         - this type parameter needs to be `Sized`
3 |     let _: [u8; sof::<T>()];
  |                       ^ doesn't have a size known at compile-time
  |
help: consider relaxing the implicit `Sized` restriction
  |
1 | fn sof<T: ?Sized>() -> T { todo!() }
  |         ^^^^^^^^

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', compiler/rustc_middle/src/ty/mod.rs:908:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
