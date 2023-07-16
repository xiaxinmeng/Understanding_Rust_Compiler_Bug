
error[E0277]: the trait bound `[X]: std::borrow::ToOwned` is not satisfied
 --> src/main.rs:5:5
  |
5 |     values: Cow<'a, [X]>,
  |     ^^^^^^^^^^^^^^^^^^^^ the trait `std::borrow::ToOwned` is not implemented for `[X]`
  |
  = help: consider adding a `where [X]: std::borrow::ToOwned` bound
  = note: required by `std::borrow::Cow`
