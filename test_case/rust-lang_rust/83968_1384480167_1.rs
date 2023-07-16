
Compiling playground v0.0.1 (/playground)
error[[E0599]](https://doc.rust-lang.org/stable/error-index.html#E0599): no method named `length` found for array `[{integer}; 3]` in the current scope
 --> src/main.rs:2:23
  |
2 |     let x = [1, 2, 3].length();
  |                       ^^^^^^ help: there is a method with a similar name: `len`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `playground` due to previous error
