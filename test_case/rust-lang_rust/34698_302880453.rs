
rustc 1.19.0-dev (5dfcd85fd 2017-05-19)
error[E0277]: the trait bound `{float}: std::ops::Rem<{integer}>` is not satisfied
 --> test.rs:2:14
  |
2 | let result = -0.25 % 2;}
  |              ^^^^^^^^^ the trait `std::ops::Rem<{integer}>` is not implemented for `{float}`
  |
  = note: no implementation for `{float} % {integer}`

error: aborting due to previous error

