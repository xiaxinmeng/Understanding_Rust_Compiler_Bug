
error[E0277]: the trait bound `u32: std::convert::From<<T as MyTrait1>::X>` is not satisfied
  --> src/main.rs:16:14
   |
6  | trait MyTrait2: MyTrait1 where <Self as MyTrait1>::X: Into<u32> {}
   |                                                       --------- required by this bound in `MyTrait2`
...
16 | fn myfunc<T: MyTrait2>(v: &T) {
   |              ^^^^^^^^ the trait `std::convert::From<<T as MyTrait1>::X>` is not implemented for `u32`
   |
   = note: required because of the requirements on the impl of `std::convert::Into<u32>` for `<T as MyTrait1>::X`
