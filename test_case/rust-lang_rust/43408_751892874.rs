
error: generic parameters may not be used in const operations
 --> src\main.rs:7:37
  |
7 | fn to_byte_array<T>() -> [u8; sof::<T>()] {
  |                                     ^ cannot perform const operation using `T`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions

error: aborting due to previous error
