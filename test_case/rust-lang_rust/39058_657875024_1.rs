
error[E0369]: cannot add `S` to `S`
 --> src/main.rs:5:15
  |
5 |     let _ = S + S;
  |             - ^ - S
  |             |
  |             S
  |
  = note: an implementation of `std::ops::Add` might be missing for `S`
