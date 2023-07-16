text
error[E0277]: the trait bound `<U as Caz>::B: Bar<<T as Caz>::A>` is not satisfied
 --> src/main.rs:7:1
  |
7 | fn test<T, U>() where T: Caz, U: Caz<A = T::A> {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Bar<<T as Caz>::A>` is not implemented for `<U as Caz>::B`
  |
  = help: consider adding a `where <U as Caz>::B: Bar<<T as Caz>::A>` bound
note: required by `Caz`
 --> src/main.rs:2:1
  |
2 | trait Caz {
  | ^^^^^^^^^
