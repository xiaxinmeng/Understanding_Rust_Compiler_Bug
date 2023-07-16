
error[E0277] the trait bound `T: SomeTrait` is not satisfied
 --> src/lib.rs:8:18
  |
8 | impl<T> Copy for SomeStruct<T> {}
  |                  ^^^^^^^^^^^^^ the trait `SomeTrait` is not implemented for `T`
  |
help: consider restricting type parameter `T`
  |
8 | impl<T: SomeTrait> Copy for SomeStruct<T> {}
  |       +++++++++++
