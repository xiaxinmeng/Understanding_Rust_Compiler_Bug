none
error[[E0015]](https://doc.rust-lang.org/stable/error-index.html#E0015): cannot call non-const operator in constant functions
 --> src/lib.rs:3:9
  |
3 |         "0" => Some(0),
  |         ^^^
  |
  = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

For more information about this error, try `rustc --explain E0015`.
error: could not compile `playground` due to previous error
