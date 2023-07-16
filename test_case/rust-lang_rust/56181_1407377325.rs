
error[E0308]: mismatched types
 --> src/lib.rs:3:27
  |
3 | pub fn bar(x: File) { foo(x) }
  |                       --- ^ expected struct `Stdio`, found struct `File`
  |                       |
  |                       arguments to this function are incorrect
  |
note: function defined here
 --> src/lib.rs:2:8
  |
2 | pub fn foo(_x: Stdio) {}
  |        ^^^ ---------
help: call `Into::into` on this expression to convert `File` into `Stdio`
  |
3 | pub fn bar(x: File) { foo(x.into()) }
  |                            +++++++

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to previous error
