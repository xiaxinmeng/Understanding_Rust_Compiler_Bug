
   Compiling playground v0.0.1 (file:///playground)
error[E0277]: the trait bound `T: std::marker::Sized` is not satisfied
 --> src/lib.rs:4:18
  |
4 |     let v : [u8; ::std::mem::size_of::<T>()] = unimplemented!();
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^ `T` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `T`
  = help: consider adding a `where T: std::marker::Sized` bound
  = note: required by `std::mem::size_of`

warning: unreachable expression
 --> src/lib.rs:5:5
  |
5 |     v
  |     ^
  |
  = note: #[warn(unreachable_code)] on by default
