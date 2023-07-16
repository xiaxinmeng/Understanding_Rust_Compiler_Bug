
4 | pub struct Test<const X: usize>(pub [usize; X]);
  |                                 ^^^^^^^^^^^^^^ `[usize; _]` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
  |
  = help: the trait `std::fmt::Debug` is not implemented for `[usize; _]`
  = note: required because of the requirements on the impl of `std::fmt::Debug` for `&[usize; _]`
  = note: required for the cast to the object type `dyn std::fmt::Debug`
