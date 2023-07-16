
error[E0308]: mismatched types
  --> src/a.rs:12:5
   |
12 |     outer(inner::<()>);
   |     ^^^^^ one type is more general than the other
   |
   = note: expected type `std::ops::FnOnce<(&(),)>`
              found type `std::ops::FnOnce<(&(),)>`
