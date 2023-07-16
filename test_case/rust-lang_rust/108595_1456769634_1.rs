bash
$ rustc test2.rs 
error[E0015]: cannot call non-const formatting macro in constant functions
 --> test2.rs:4:33
  |
4 |     let _  = format_args!("{}", "literal");
  |                                 ^^^^^^^^^
  |
  = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
  = note: this error originates in the macro `format_args` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
 --> test2.rs:4:27
  |
4 |     let _  = format_args!("{}", "literal");
  |                           ^^^^

note: erroneous constant used
 --> test2.rs:4:33
  |
4 |     let _  = format_args!("{}", "literal");
  |                                 ^^^^^^^^^
  |
  = note: this note originates in the macro `format_args` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0015`.
