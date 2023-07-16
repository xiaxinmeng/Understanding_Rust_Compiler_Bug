
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> src/main.rs:17:41
   |
17 |     fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
   |                                         ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: within `Foo`, the trait `Sized` is not implemented for `[u8]`
note: required because it appears within the type `Foo`
  --> src/main.rs:10:12
   |
10 | pub struct Foo {
   |            ^^^
note: required by a bound in `Result`
  --> /rustc/f3126500f25114ba4e0ac3e76694dd45a22de56d/library/core/src/result.rs:503:1
