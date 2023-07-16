
---- src/lib.rs - foo (line 3) stdout ----
error[E0369]: binary operation `+` cannot be applied to type `&str`
 --> src/lib.rs:5:18
  |
3 |     let _x = "t" + 10;
  |              --- ^ -- {integer}
  |              |
  |              &str
  |
  = note: an implementation of `std::ops::Add` might be missing for `&str`

error: aborting due to previous error
