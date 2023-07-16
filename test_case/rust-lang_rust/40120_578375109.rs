
error[E0277]: the trait bound `i32: Bar` is not satisfied
 --> src/main.rs:6:5
  |
1 | trait Foo { fn foo() {} }
  |             -------- required by `Foo::foo`
...
6 |     <i32 as Foo>::foo();
  |     ^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `i32`
  |
  = note: required because of the requirements on the impl of `Foo` for `i32`
