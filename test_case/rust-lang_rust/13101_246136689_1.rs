
error[E0277]: the trait bound `NotEq: std::cmp::Eq` is not satisfied
 --> ../../test.rs:9:2
  |
9 |     a: NotEq,
  |     ^^^^^^^^ trait `NotEq: std::cmp::Eq` not satisfied
  |
  = note: required by `std::cmp::AssertParamIsEq`
