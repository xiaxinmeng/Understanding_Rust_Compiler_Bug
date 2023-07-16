
error[E0308]: mismatched types
  --> src/main.rs:41:17
   |
41 |     let _: u8 = b.meta();
   |                 ^^^^^^^^ expected u8, found associated type
   |
   = note: expected type `u8`
              found type `<usize as Trait>::Meta`
