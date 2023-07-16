
error[E0081]: discriminant value `-1` assigned more than once
 --> tst.rs:4:5
  |
2 |     First = -1,
  |             -- first assignment of `-1`
3 |     Second = -2,
4 |     Last,
  |     ^^^^
  |
note: because `Last` has no explicit discriminant, it is being assigned the increment of the previous discriminant
 --> tst.rs:4:5
  |
4 |     Last,
  |     ^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0081`.
