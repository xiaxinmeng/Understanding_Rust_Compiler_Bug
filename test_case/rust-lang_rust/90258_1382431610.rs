
error[E0599]: no method named `slice` found for array `[{integer}; 32]` in the current scope
 --> aaa.rs:2:21
  |
2 |     let _ = [0; 32].slice();
  |                     ^^^^^ help: there is a method with a similar name: `as_slice`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
