console
$ rustc test.rs
error[E0518]: attribute should be applied to function
 --> test.rs:2:2
  |
2 |     #[inline] || { };
  |     ^^^^^^^^^ ------ not a function

error: aborting due to previous error

For more information about this error, try `rustc --explain E0518`.
