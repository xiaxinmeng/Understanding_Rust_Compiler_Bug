
error[E0277]: the trait bound `u8: MyTrait` is not satisfied
  --> src/lib.rs:10:23
   |
1  | trait MyTrait {
   | ------------- required by this bound in `MyTrait`
...
10 |     type OtherItem = (<u8 as MyTrait>::Item, bool);
   |                       ^^^^^^^^^^^^^^^^^^^^^ the trait `MyTrait` is not implemented for `u8`
