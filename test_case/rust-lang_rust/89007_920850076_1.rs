rust
error[E0277]: cannot add `u32` to `u32`
  --> ...\test3.rs:15:20
   |
15 |     const X: u32 = foo(1_u32, 1_u32);
   |                    ^^^ no implementation for `u32 + u32`
   |
   = help: the trait `Add` is not implemented for `u32`
note: required by a bound in `foo`
  --> ...\test3.rs:10:43
   |
10 | const fn foo<T>(a: T, b: T) -> T where T: ~const Add<Output=T> {
   |                                           ^^^^^^^^^^^^^^^^^^^^ required by this bound in `foo`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
14 | fn main() where u32: Add {
   |           ++++++++++++++
