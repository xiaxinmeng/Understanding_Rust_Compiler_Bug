
error[E0308]: mismatched types
  --> src/lib.rs:11:21
   |
11 |     const N: C::M = 4u8;
   |                     ^^^ expected associated type, found u8
   |
   = note: expected type `<C as O>::M`
              found type `u8`
