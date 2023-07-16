
error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> src/lib.rs:5:40
  |
5 |     fn f() -> [u8; std::mem::size_of::<Self>()];
  |                                        ^^^^ doesn't have a size known at compile-time
  |
note: required by a bound in `std::mem::size_of`
 --> /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/core/src/mem/mod.rs:308:1
help: consider further restricting `Self`
  |
5 |     fn f() -> [u8; std::mem::size_of::<Self>()] where Self: Sized;
  |                                                 +++++++++++++++++

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error
