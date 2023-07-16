
error[[E0275]](https://doc.rust-lang.org/nightly/error-index.html#E0275): overflow evaluating the requirement `f64: Mul<&Point<_>>`
  --> src/lib.rs:36:46
   |
36 | pub const DEGREES: Angle = Quantity(RADIANS.0*PI/180.0, detail::AngleUnit{});
   |                                              ^
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`playground`)
note: required for `f64` to implement `Mul<&Point<Point<_>>>`
  --> src/lib.rs:18:13
   |
18 | impl<'b, T> Mul<&'b Point<T>> for f64 where f64: Mul<&'b T> {
   |             ^^^^^^^^^^^^^^^^^     ^^^
   = note: 127 redundant requirements hidden
   = note: required for `f64` to implement `Mul<&Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<_>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`

For more information about this error, try `rustc --explain E0275`.
error: could not compile `playground` due to previous error
