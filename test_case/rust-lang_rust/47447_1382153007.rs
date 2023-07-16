rust
Compiling playground v0.0.1 (/playground)
error[[E0425]](https://doc.rust-lang.org/stable/error-index.html#E0425): cannot find function `bar` in this scope
 --> src/main.rs:5:17
  |
5 |       /*Self::*/bar()
  |                 ^^^ not found in this scope
  |
help: consider using the associated function
  |
5 |       /*Self::*/Self::bar()
  |                 ~~~~~~~~~

For more information about this error, try `rustc --explain E0425`.
error: could not compile `playground` due to previous error
