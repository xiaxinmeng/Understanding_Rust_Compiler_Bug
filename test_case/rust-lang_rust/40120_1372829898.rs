
error[E0277]: the trait bound `i32: Bar` is not satisfied
 --> src/main.rs:6:5
  |
6 |     <i32 as Foo>::foo();
  |     ^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `i32`
  |
note: required for `i32` to implement `Foo`
 --> src/main.rs:3:14
  |
3 | impl<T: Bar> Foo for T {}
  |              ^^^     ^
