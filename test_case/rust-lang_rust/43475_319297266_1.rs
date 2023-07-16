
Compiling playground v0.0.1 (file:///playground)
error[E0277]: the trait bound `<T as Test>::T: Param<u8>` is not satisfied
  --> src/main.rs:15:1
   |
15 | / fn test2<T: Test<Item = u8>>(p: T::Item) -> T::T {
16 | |     T::T::param(p)
17 | | }
   | |_^ the trait `Param<u8>` is not implemented for `<T as Test>::T`
   |
   = help: consider adding a `where <T as Test>::T: Param<u8>` bound
   = note: required by `Test`

error: aborting due to previous error

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
