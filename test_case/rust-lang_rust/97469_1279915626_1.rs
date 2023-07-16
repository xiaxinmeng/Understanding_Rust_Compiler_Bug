
Compiling playground v0.0.1 (/playground)
error: `let` expressions are not supported here
 --> src/lib.rs:4:18
  |
4 |     let res = if let Some(u1) = o1 && let Some(u2) = o2 && b1 = b2
  |                  ^^^^^^^^^^^^^^^^^
  |
  = note: only supported directly in conditions of `if` and `while` expressions

error: `let` expressions are not supported here
 --> src/lib.rs:4:39
  |
4 |     let res = if let Some(u1) = o1 && let Some(u2) = o2 && b1 = b2
  |                                       ^^^^^^^^^^^^^^^^^
  |
  = note: only supported directly in conditions of `if` and `while` expressions

error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/lib.rs:4:18
  |
4 |     let res = if let Some(u1) = o1 && let Some(u2) = o2 && b1 = b2
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
  |
help: you might have meant to compare for equality
  |
4 |     let res = if let Some(u1) = o1 && let Some(u2) = o2 && b1 == b2
  |                                                                +

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to 3 previous errors
