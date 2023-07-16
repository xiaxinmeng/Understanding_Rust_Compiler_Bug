
error[E0277]: the trait bound `{float}: std::ops::AddAssign<{integer}>` is not satisfied
 --> <anon>:3:5
  |
3 |     x += 1;
  |     ^^^^^^ the trait `std::ops::AddAssign<{integer}>` is not implemented for `{float}`
  |
  = help: the following implementations were found:
  = help:   <f64 as std::ops::AddAssign>
  = help:   <f32 as std::ops::AddAssign>
  = help:   <u64 as std::ops::AddAssign>
  = help:   <std::time::Duration as std::ops::AddAssign>
  = help: and 24 others
