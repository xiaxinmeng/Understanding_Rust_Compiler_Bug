
error[E0308]: mismatched types
  --> src/lib.rs:11:17
   |
11 |     let c: () = thing;
   |                 ^^^^^ expected (), found associated type
   |
   = note: expected type `()`
              found type `<() as Tr>::Assoc`
