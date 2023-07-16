
error[E0308]: mismatched types
  --> src/main.rs:15:5
   |
15 |     foo({ async fn baz(f: &u8) -> u8 { *f } |f| baz(f) });
   |     ^^^ one type is more general than the other
   |
   = note: expected type `std::ops::FnOnce<(&'a u8,)>`
              found type `std::ops::FnOnce<(&u8,)>`
