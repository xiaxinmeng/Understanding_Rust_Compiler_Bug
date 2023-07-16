
warning: constant is never used: `A`
 --> src/main.rs:4:1
  |
4 | const A: () = panic!("panic while evaluating const");
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

error: any use of this value will cause an error
 --> src/main.rs:4:15
  |
4 | const A: () = panic!("panic while evaluating const");
  | --------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
  |               |
  |               the evaluated program panicked at 'panic while evaluating const', src/main.rs:4:15
  |
  = note: `#[deny(const_err)]` on by default
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
