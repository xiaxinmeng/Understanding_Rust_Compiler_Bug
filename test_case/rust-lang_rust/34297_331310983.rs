
error[E0277]: the trait bound `{integer}: std::ops::AddAssign<{float}>` is not satisfied
 --> src/main.rs:3:7
  |
3 |     x += 1.0;
  |       ^^ no implementation for `{integer} += {float}`
  |
  = help: the trait `std::ops::AddAssign<{float}>` is not implemented for `{integer}`
