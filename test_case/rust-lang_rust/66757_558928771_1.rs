console
error[E0277]: the trait bound `(): std::error::Error` is not satisfied
 --> src/main.rs:2:5
  |
2 |     Box::new(e)
  |     ^^^^^^^^^^^ the trait `std::error::Error` is not implemented for `()`
  |
  = note: required for the cast to the object type `dyn std::error::Error`
