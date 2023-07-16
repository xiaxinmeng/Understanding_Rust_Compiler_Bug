
error[E0119]: conflicting implementations of trait `std::error::Error` for type `&MyError`:
  --> src\lib.rs:13:1
   |
13 | impl<'a> Error for &'a MyError {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: conflicting implementation in crate `std`:
           - impl<'a, T> std::error::Error for &'a T
             where T: std::error::Error, T: ?Sized;
