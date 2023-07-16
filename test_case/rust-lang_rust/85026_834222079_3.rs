
error[E0118]: no nominal type found for inherent implementation
 --> src/lib.rs:4:6
  |
4 | impl dyn X {}
  |      ^^^^^ impl requires a nominal type
  |
  = note: either implement a trait on it or create a newtype to wrap it instead
