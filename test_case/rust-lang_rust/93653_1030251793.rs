plain
---- src/boxed.rs - boxed::Box<T,A>::take (line 599) stdout ----
error[E0433]: failed to resolve: use of undeclared type `Bow`
  --> src/boxed.rs:608:19
   |
12 | let boxed_value = Bow::write(allocation, value);
   |                   ^^^ use of undeclared type `Bow`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T,A>::take (line 589) stdout ----
error[E0308]: mismatched types
 --> src/boxed.rs:594:10
  |
8 | unsafe { Box::take(value).1.assume_init() }
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found struct `Box`
  = note: expected unit type `()`
                found struct `Box<String>`
help: consider using a semicolon here
  |
  |
8 | unsafe { Box::take(value).1.assume_init(); }
help: try adding a return type
  |
  |
5 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_boxed_rs_589_0() -> Box<String> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
