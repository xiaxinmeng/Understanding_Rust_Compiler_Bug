
error[E0369]: binary operation `==` cannot be applied to type `fn() {main::dummy}`
 --> src/main.rs:9:5
  |
9 |     dummy  == dummy;  // Err
  |     ^^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialEq` might be missing for `fn() {main::dummy}`
