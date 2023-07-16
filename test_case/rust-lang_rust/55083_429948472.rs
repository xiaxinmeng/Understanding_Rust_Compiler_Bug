
error[E0275]: overflow evaluating the requirement `_: std::marker::Sized`
  --> src/lib.rs:36:46
   |
36 | pub const DEGREES: Angle = Quantity(RADIANS.0*PI/180.0, detail::AngleUnit{});
   |                                              ^
   |
   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
   = note: required because of the requirements on the impl of `std::ops::Mul<&Point<_>>` for `f64`
   = note: required because of the requirements on the impl of `std::ops::Mul<&Point<Point<_>>>` for `f64`
   = note: required because of the requirements on the impl of `std::ops::Mul<&Point<Point<Point<_>>>>` for `f64`
   = note: required because of the requirements on the impl of `std::ops::Mul<&Point<Point<Point<Point<_>>>>>` for `f64`
   = note: required because of the requirements on the impl of `std::ops::Mul<&Point<Point<Point<Point<Point<_>>>>>>` for `f64`
   = note: required because of the requirements on the impl of `std::ops::Mul<&Point<Point<Point<Point<Point<Point<_>>>>>>>` for `f64`
<... cont ...>
