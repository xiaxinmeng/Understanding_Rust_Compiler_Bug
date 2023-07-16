
error[E0277]: the trait bound `U: std::marker::Sized` is not satisfied
  --> src/lib.rs:12:5
   |
12 |     const VAL: Self::T = 5;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `U` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `U`
   = help: consider adding a `where U: std::marker::Sized` bound
   = note: required because of the requirements on the impl of `Const<U>` for `()`

error[E0277]: the trait bound `U: EmptyTrait` is not satisfied
  --> src/lib.rs:12:5
   |
12 |     const VAL: Self::T = 5;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `EmptyTrait` is not implemented for `U`
   |
   = help: consider adding a `where U: EmptyTrait` bound
   = note: required because of the requirements on the impl of `Const<U>` for `()`
