rust
error[E0277]: the trait bound `(u8, u64): Default` is not satisfied
  --> ...\test1.rs:10:27
   |
10 |     const X2: (u8, u64) = foo(); // Err
   |                           ^^^ the trait `Default` is not implemented for `(u8, u64)`
   |
note: required by a bound in `foo`
  --> ...\test1.rs:4:17
   |
4  | const fn foo<T: ~const Default + Copy>() -> T {
   |                 ^^^^^^^^^^^^^^ required by this bound in `foo`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
8  | fn main() where (u8, u64): Default {
   |           ++++++++++++++++++++++++
