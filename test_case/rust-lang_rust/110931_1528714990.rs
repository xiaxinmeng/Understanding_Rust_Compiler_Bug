rust
error[[E0637]](https://doc.rust-lang.org/stable/error_codes/E0637.html): `'_` cannot be used here
 --> src/lib.rs:1:39
  |
1 | struct Foo<const N: usize = { let a: &'_ () = &(); 10 }>;
  |                                       ^^ `'_` is a reserved lifetime name

For more information about this error, try `rustc --explain E0637`.
error: could not compile `playground` due to previous error
