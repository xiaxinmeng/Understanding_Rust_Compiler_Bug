
error[E0277]: the trait bound `T: SomeTrait` is not satisfied in `SomeStruct<T>`
  --> src/lib.rs:10:9
   |
10 | impl<T> Clone for SomeStruct<T> {
   |         ^^^^^ within `SomeStruct<T>`, the trait `SomeTrait` is not implemented for `T`
   |
note: required because it appears within the type `SomeStruct<T>`
  --> src/lib.rs:5:8
   |
5  | struct SomeStruct<T: SomeTrait>(T::SomeAssociatedType);
   |        ^^^^^^^^^^
note: required by a bound in `Clone`
  --> /rustc/659e169d37990b9c730a59a96081f2ef7afbe8f1/library/core/src/clone.rs:110:1
help: consider restricting type parameter `T`
   |
10 | impl<T: SomeTrait> Clone for SomeStruct<T> {
   |       +++++++++++

error[E0277]: the trait bound `T: SomeTrait` is not satisfied
  --> src/lib.rs:11:14
   |
11 |     fn clone(&self) -> Self {
   |              ^^^^^ the trait `SomeTrait` is not implemented for `T`
   |
note: required by a bound in `SomeStruct`
  --> src/lib.rs:5:22
   |
5  | struct SomeStruct<T: SomeTrait>(T::SomeAssociatedType);
   |                      ^^^^^^^^^ required by this bound in `SomeStruct`
help: consider restricting type parameter `T`
   |
10 | impl<T: SomeTrait> Clone for SomeStruct<T> {
   |       +++++++++++
