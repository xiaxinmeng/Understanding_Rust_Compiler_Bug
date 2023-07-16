
error[E0308]: mismatched types
  --> src/lib.rs:11:21
   |
11 |     const N: C::M = 4u8;
   |                     ^^^ expected associated type, found u8
   |
   = note: expected type `<C as O>::M`
              found type `u8`
   = note: consider constraining the associated type `<C as O>::M` to `u8` or calling a method that returns `<C as O>::M`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
