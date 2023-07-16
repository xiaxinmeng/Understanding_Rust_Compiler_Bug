
error[E0277]: the trait bound `<<Self as A>::T as A>::T: B` is not satisfied
 --> /home/../test.rs:7:15
  |
7 | trait B: A<T: B> {}
  |               ^ the trait `B` is not implemented for `<<Self as A>::T as A>::T`
  |
note: required by a bound in `B`
 --> /home/../test.rs:7:15
  |
7 | trait B: A<T: B> {}
  |               ^ required by this bound in `B`
help: consider further restricting the associated type
  |
7 | trait B: A<T: B> where <<Self as A>::T as A>::T: B {}
  |                  +++++++++++++++++++++++++++++++++
