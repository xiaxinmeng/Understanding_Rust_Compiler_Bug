
error[E0277]: `T` doesn't implement `Debug`
 --> main.rs:9:25
  |
9 |         print!("{:?},", self.m);
  |                         ^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |
  = note: required by `std::fmt::Debug::fmt`
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
  |
7 | impl<T: <T as std::fmt::Debug>> Test<T> {
  |       ^^^^^^^^^^^^^^^^^^^^^^^^

