
error[E0308]: mismatched types
 --> src/a.rs:3:29
  |
3 |     vec![1,2,3].into_iter().find(suitable);
  |                             ^^^^ one type is more general than the other
  |
  = note: expected type `std::ops::FnOnce<(&i32,)>`
             found type `std::ops::FnOnce<(&i32,)>`
