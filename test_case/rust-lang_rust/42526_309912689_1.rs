
error[E0277]: the trait bound `MyError: std::convert::From<NoneError>` is not satisfied
  --> foo.rs:32:13
   |
32 |     let a = method_returning_option_i32()?;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<NoneError>` is not implemented for `MyError`
   |
   = note: required by `std::convert::From::from`

error: aborting due to previous error(s)
