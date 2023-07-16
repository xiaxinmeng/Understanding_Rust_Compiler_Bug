rust
error[E0277]: the trait bound `U: std::marker::Sized` is not satisfied
  --> src/main.rs:11:5
   |
11 |     const VAL: Self::T = (U::VAL, U::VAL);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `U` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `U`
   = help: consider adding a `where U: std::marker::Sized` bound
   = note: required because of the requirements on the impl of `Const` for `MakeTuple<U>`

error[E0277]: the trait bound `U: Const` is not satisfied
  --> src/main.rs:11:5
   |
11 |     const VAL: Self::T = (U::VAL, U::VAL);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Const` is not implemented for `U`
   |
   = help: consider adding a `where U: Const` bound
   = note: required because of the requirements on the impl of `Const` for `MakeTuple<U>`
