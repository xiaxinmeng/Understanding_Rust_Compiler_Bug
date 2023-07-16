
error[E0308]: mismatched types
  --> src/lib.rs:52:17
   |
52 |         .filter(apply(second, i))
   |                 ^^^^^ one type is more general than the other
   |
   = note: expected type `std::ops::FnOnce<(&&(&str, &dyn std::ops::Fn(i32) -> bool),)>`
              found type `std::ops::FnOnce<(&&(&str, &dyn std::ops::Fn(i32) -> bool),)>`
