
error[E0229]: associated type bindings are not allowed here
 --> src/main.rs:6:17
  |
6 | impl Fn(u32) -> u32 for S {
  |                 ^^^ associated type not allowed here

error[E0277]: expected a `FnMut<(u32,)>` closure, found `S`
  --> src/main.rs:6:6
   |
6  | impl Fn(u32) -> u32 for S {
   |      ^^^^^^^^^^^^^^ expected an `FnMut<(u32,)>` closure, found `S`
   |
   = help: the trait `FnMut<(u32,)>` is not implemented for `S`
note: required by a bound in `Fn`
