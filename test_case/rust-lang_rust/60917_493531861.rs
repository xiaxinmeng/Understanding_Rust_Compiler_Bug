
error[E0277]: `?` couldn't convert the error to `Error2`
  --> src/main.rs:11:10
   |
11 |     foo()?;
   |          ^ the trait `std::convert::From<Error1>` is not implemented for `Error2`
   |
   = note: required by `std::convert::From::from`
