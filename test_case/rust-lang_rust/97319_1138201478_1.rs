
error[E0081]: discriminant value `-2` assigned more than once
 --> tst.rs:6:5
  |
4 |     Second = -2,
  |              -- first assignment of `-2`
5 |     Third = -3,
6 |     Last
  |     ^^^^ second assignment of `-2`
  |
  = note: because `Last` has no explicit discriminant, it gets assigned the increment of the previous discriminant

error: aborting due to previous error
