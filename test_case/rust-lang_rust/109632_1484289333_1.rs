text
Compiling playground v0.0.1 (/playground)
error[E0277]: the trait bound `<T as Trait1>::Assoc1: MyIterator` is not satisfied
  --> src/lib.rs:28:5
   |
28 |     fn my_nth() {}
   |     ^^^^^^^^^^^ the trait `MyIterator` is not implemented for `<T as Trait1>::Assoc1`
   |
note: required for `GenericType<T>` to implement `MyIterator`
  --> src/lib.rs:22:17
   |
22 | impl<T: Trait1> MyIterator for GenericType<T>
   |                 ^^^^^^^^^^     ^^^^^^^^^^^^^^
23 | where
24 |     <Self as Trait2>::Assoc2: MyIterator,
   |                               ---------- unsatisfied trait bound introduced here
help: consider further restricting the associated type
   |
28 |     fn my_nth() where <T as Trait1>::Assoc1: MyIterator {}
   |                 +++++++++++++++++++++++++++++++++++++++

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error
