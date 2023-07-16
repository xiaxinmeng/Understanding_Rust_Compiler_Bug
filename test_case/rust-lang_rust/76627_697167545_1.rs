rust
error[E0308]: mismatched types
 --> src/main.rs:3:5
  |
3 |     x(c);
  |     ^ one type is more general than the other
  |
  = note: expected type `std::ops::FnOnce<(&'a (),)>`
             found type `std::ops::FnOnce<(&(),)>`
