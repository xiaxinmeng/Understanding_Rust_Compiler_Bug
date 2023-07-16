rust
error[E0277]: the trait bound `i32: Foo` is not satisfied
  |
  |     <i32 as Foo>::foo();
  |     ^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `i32`
  |
  = note: the trait bound could be satisfied by implementing `Bar` for `i32`
  = note: required by `Foo::foo`
