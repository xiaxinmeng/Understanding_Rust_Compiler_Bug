
error[E0277]: the trait bound `BImpl: B` is not satisfied
 --> test.rs:5:13
  |
5 | impl<BImpl> A<BImpl> for AImpl {
  |             ^^^^^^^^ the trait `B` is not implemented for `BImpl`
  |
  = help: consider adding a `where BImpl: B` bound
  = note: required by `A`

error: aborting due to previous error
