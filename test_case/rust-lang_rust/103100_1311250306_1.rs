
error[E0277]: the trait bound `String: From<{integer}>` is not satisfied
 --> src/main.rs:6:9
  |
6 |     foo(42);
  |     --- ^^ the trait `From<{integer}>` is not implemented for `String`
  |     |
  |     required by a bound introduced by this call
