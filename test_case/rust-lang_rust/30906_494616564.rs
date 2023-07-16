
error[E0308]: mismatched types
  --> src/lib.rs:20:5
   |
20 |     test(Compose(f, |_| {}));
   |     ^^^^ one type is more general than the other
   |
   = note: expected type `std::ops::FnOnce<(&'x str,)>`
              found type `std::ops::FnOnce<(&str,)>`
