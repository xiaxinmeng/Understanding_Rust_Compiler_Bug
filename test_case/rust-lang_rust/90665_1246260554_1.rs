
error[E0277]: the trait bound `(): Bound` is not satisfied
  --> src/main.rs:14:5
   |
14 |     <() as Foo<_>>::foo();
   |     ^^^^^^^^^^^^^^^^^^^ the trait `Bound` is not implemented for `()`
   |
note: required because of the requirements on the impl of `Foo<u32>` for `()`
  --> src/main.rs:7:16
   |
7  | impl<T: Bound> Foo<u32> for T {
   |                ^^^^^^^^     ^
