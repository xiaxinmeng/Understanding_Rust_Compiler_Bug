
error[E0308]: mismatched types
 --> src/main.rs:8:55
  |
8 |     std::env::var("NONEXISTENT").unwrap_or_else(|err| error(err));
  |                                                       ^^^^^^^^^^ expected struct `std::string::String`, found `()`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `exclamation-error` due to previous error
