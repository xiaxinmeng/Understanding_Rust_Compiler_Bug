
error[E0204]: the trait `Copy` may not be implemented for this type
  --> src/main.rs:12:10
   |
12 | #[derive(Copy, Clone)]
   |          ^^^^
...
15 |     b: Struct<i64>,
   |     -------------- this field does not implement `Copy`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0204`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
