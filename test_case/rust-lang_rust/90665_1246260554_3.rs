
error[E0277]: the trait bound `(): Foo<_>` is not satisfied
  --> src/main.rs:20:5
   |
20 |     <() as Foo<_>>::foo();
   |     ^^^^^^^^^^^^^^^^^^^ the trait `Foo<_>` is not implemented for `()`
