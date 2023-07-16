
error[E0277]: the trait bound `for<'x> i32: From<<&'x i32 as Add>::Output>` is not satisfied
  --> src/main.rs:23:16
   |
23 |     assert_eq!(ref_add_hrtb(&a, &b), 5i32); // E0277
   |                ^^^^^^^^^^^^ the trait `for<'x> From<<&'x i32 as Add>::Output>` is not implemented for `i32`
   |
note: required by a bound in `ref_add_hrtb`
  --> src/main.rs:13:8
   |
11 | fn ref_add_hrtb<T>(a: &T, b: &T) -> T
   |    ------------ required by a bound in this
12 | where
13 |     T: for<'x> From<<&'x T as Add>::Output>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ref_add_hrtb`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
19 | fn main() where i32: for<'x> From<<&'x i32 as Add>::Output> {
   |           +++++++++++++++++++++++++++++++++++++++++++++++++

For more information about this error, try `rustc --explain E0277`.
