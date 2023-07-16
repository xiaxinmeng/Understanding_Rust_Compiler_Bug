
error[[E0562]](https://doc.rust-lang.org/nightly/error_codes/E0562.html): `impl Trait` only allowed in function and inherent method return types, not in field types
 --> src/lib.rs:7:8
  |
7 |     x: impl Into<String>,
  |        ^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0562`.
error: could not compile `playground` (lib) due to previous error
