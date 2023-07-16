
error[E0277]: the trait bound `<U as Caz>::B: Bar<<T as Caz>::A>` is not satisfied
 --> src/lib.rs:7:1
  |
2 | trait Caz {
  | --------- required by `Caz`
...
7 | fn test<T, U>() where T: Caz, U: Caz<A = T::A> {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-^^
  | |                                             |
  | |                                             help: consider further restricting the associated type: `, <U as Caz>::B: Bar<<T as Caz>::A>`
  | the trait `Bar<<T as Caz>::A>` is not implemented for `<U as Caz>::B`
