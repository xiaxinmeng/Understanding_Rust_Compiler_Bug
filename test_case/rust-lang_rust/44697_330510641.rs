
   Compiling playground v0.0.1 (file:///playground)
error[E0433]: failed to resolve. Could not find `std` in `std`
 --> src/main.rs:1:42
  |
1 | fn foo(_: &[std::string::String]) -> Vec<std::std::string::String> {
  |                                          ^^^^^^^^^^^^^^^^^^^^^^^^ Could not find `std` in `std`

error[E0277]: the trait bound `[std::string::String]: std::marker::Sized` is not satisfied
 --> src/main.rs:6:9
  |
6 |     let x = foo(&Vec::new());
  |         ^ `[std::string::String]` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `[std::string::String]`
  = note: all local variables must have a statically known size

error: aborting due to 2 previous errors

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
