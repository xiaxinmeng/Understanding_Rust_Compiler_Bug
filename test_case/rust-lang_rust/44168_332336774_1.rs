
error[E0277]: the trait bound `T: HasLength` is not satisfied
 --> src/main.rs:5:42
  |
5 | fn foo<T: Copy + HasLength>(x: T) -> [T; <T as HasLength>::LENGTH] { [x; <T as HasLength>::LENGTH] }
  |                                          ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `HasLength` is not implemented for `T`
  |
  = help: consider adding a `where T: HasLength` bound
  = note: required by `HasLength::LENGTH`
