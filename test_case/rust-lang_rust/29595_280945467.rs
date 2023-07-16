
rustc 1.17.0-nightly (306035c21 2017-02-18)
error[E0277]: the trait bound `u8: Tr` is not satisfied
 --> <anon>:8:17
  |
8 |     let a: u8 = Tr::C;
  |                 ^^^^^ the trait `Tr` is not implemented for `u8`
  |
  = note: required by `Tr::C`

error: aborting due to previous error
