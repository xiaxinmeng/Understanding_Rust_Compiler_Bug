
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
 --> src/lib.rs:9:9
  |
7 | impl<i64> DoIt<i64> for Dummy {
  |      --- this type parameter
8 |     fn doit(&self, data: i64) -> i64 {
  |                                  --- expected `i64` because of return type
9 |         123
  |         ^^^ expected type parameter `i64`, found integer
  |
  = note: expected type parameter `i64`
                       found type `{integer}`

For more information about this error, try `rustc --explain E0308`.
