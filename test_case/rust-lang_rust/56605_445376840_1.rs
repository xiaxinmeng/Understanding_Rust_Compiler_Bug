
error[E0277]: the trait bound `T: ArrayGenerator` is not satisfied
 --> src/main.rs:5:32
  |
5 | fn generate<T>(obj: T) -> [u8; <T as ArrayGenerator>::LEN] where T: ArrayGenerator {
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ArrayGenerator` is not implemented for `T`
  |
  = help: consider adding a `where T: ArrayGenerator` bound
note: required by `ArrayGenerator::LEN`
 --> src/main.rs:2:3
  |
2 |   const LEN: usize;
  |   ^^^^^^^^^^^^^^^^^

error: aborting due to previous error
