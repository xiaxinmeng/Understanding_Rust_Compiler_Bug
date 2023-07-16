
error[[E0373]](https://doc.rust-lang.org/nightly/error_codes/E0373.html): closure may outlive the current function, but it borrows `x`, which is owned by the current function
 --> src/lib.rs:2:5
  |
2 |     || x
  |     ^^ - `x` is borrowed here
  |     |
  |     may outlive borrowed value `x`
  |
note: closure is returned here
 --> src/lib.rs:2:5
  |
2 |     || x
  |     ^^^^
help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
  |
2 |     move || x
  |     ++++

For more information about this error, try `rustc --explain E0373`.
error: could not compile `playground` (lib) due to previous error
