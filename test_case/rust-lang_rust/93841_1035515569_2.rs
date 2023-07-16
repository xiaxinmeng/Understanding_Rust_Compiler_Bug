
cargo test --doc
   Compiling rust-doc v0.1.0 (G:\workspace\rust-doc)
    Finished test [unoptimized + debuginfo] target(s) in 7.25s
   Doc-tests rust-doc

running 1 test
test src\lib.rs - (line 1) ... FAILED

failures:

---- src\lib.rs - (line 1) stdout ----
error[E0308]: mismatched types
 --> src\lib.rs:5:1
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_lib_rs_1_0() -> Result<(), impl core::fmt::Debug> {
  |                                                                                      --------------------- the expected opaque type
...
6 | Ok::<(), std::boxed::Box<dyn std::error::Error + 'static>>(())
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::fmt::Error`, found struct `Box`
  |
  = note: expected opaque type `impl Debug`
                  found struct `Box<dyn std::error::Error>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
