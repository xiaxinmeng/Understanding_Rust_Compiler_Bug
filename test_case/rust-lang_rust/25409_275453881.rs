
rustc 1.14.0 (e8a012324 2016-12-16)
error: main function not found

error[E0277]: the trait bound `Self: std::marker::Sized` is not satisfied
 --> <anon>:1:1
  |
1 | trait Bound where u8: Into<Self> {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Sized` is not implemented for `Self`
  |
  = help: consider adding a `where Self: std::marker::Sized` bound
  = note: required by `std::convert::Into`

error[E0277]: the trait bound `B: std::convert::From<u8>` is not satisfied
  --> <anon>:8:1
   |
8  | fn test<B: Bound>() -> B {
   | ^ the trait `std::convert::From<u8>` is not implemented for `B`
   |
   = help: consider adding a `where B: std::convert::From<u8>` bound
   = note: required because of the requirements on the impl of `std::convert::Into<B>` for `u8`
   = note: required by `Bound`

error: aborting due to 2 previous errors
