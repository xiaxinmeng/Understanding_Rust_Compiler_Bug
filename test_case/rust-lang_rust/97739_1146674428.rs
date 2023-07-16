plain
running 59 tests
.........ii....................F...........................
failures:

---- src/let_underscore.rs - let_underscore::LET_UNDERSCORE_LOCK (line 57) stdout ----
error[E0433]: failed to resolve: use of undeclared crate or module `thread`
 --> src/let_underscore.rs:61:1
  |
6 | thread::spawn(move || {
  | ^^^^^^ use of undeclared crate or module `thread`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
