
error[[E0204]](https://doc.rust-lang.org/nightly/error-index.html#E0204): the trait `Copy` may not be implemented for this type
  [--> src/lib.rs:9:17
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018#)   |
9  | #[derive(Debug, Copy, Clone)]
   |                 ^^^^
10 | pub struct AABB<T>{
11 |     pub loc: Vector2<T>,
   |     ------------------- this field does not implement `Copy`
12 |     pub size: Vector2<T>
   |     -------------------- this field does not implement `Copy`
   |
note: the `Copy` impl for `Vector2<T>` requires that `T: Debug`
  [--> src/lib.rs:11:5
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018#)   |
11 |     pub loc: Vector2<T>,
   |     ^^^^^^^^^^^^^^^^^^^
note: the `Copy` impl for `Vector2<T>` requires that `T: Debug`
  [--> src/lib.rs:12:5
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018#)   |
12 |     pub size: Vector2<T>
   |     ^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
