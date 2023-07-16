
error: any use of this value will cause an error
 --> src/main.rs:6:21
  |
6 |     const VOID: ! = panic!();
  |     ----------------^^^^^^^^-
  |                     |
  |                     the evaluated program panicked at 'explicit panic', src/main.rs:6:21
  |
  = note: `#[deny(const_err)]` on by default
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0080]: erroneous constant used
  --> src/main.rs:10:5
   |
10 |     PrintName::VOID;
   |     ^^^^^^^^^^^^^^^ referenced constant has errors
