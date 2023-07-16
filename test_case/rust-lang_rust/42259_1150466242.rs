
error[E0204]: the trait `Copy` may not be implemented for this type
  --> src/main.rs:12:10
   |
12 | #[derive(Copy, Clone)]
   |          ^^^^
...
15 |     b: Struct<i64>,
   |     -------------- this field does not implement `Copy`
   |
note: the `Copy` impl for `Struct<i64>` requires that `i64: Trait`
  --> src/main.rs:15:8
   |
15 |     b: Struct<i64>,
   |        ^^^^^^^^^^^
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
