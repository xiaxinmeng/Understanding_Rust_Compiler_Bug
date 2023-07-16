
Ty: Discr {
    val: 255,
    ty: i8,
}
error[E0081]: discriminant value `255` already exists
 --> tst.rs:5:5
  |
3 |     First = -1,
  |             -- first use of `255`
4 |     Second = -2,
5 |     Last
  |     ^^^^ enum already has `255`

error: aborting due to previous error
