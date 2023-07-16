
error[E0221]: ambiguous associated type `Err` in bounds of `Self`
  --> src/test/compile-fail/E0221.rs:32:16
   |
30 |     type Err: T3; //~ NOTE: ambiguous `Err` from `My`
   |     ------------- ambiguous `Err` from `My`
31 |     fn test() {
32 |         let _: Self::Err;
   |                ^^^^^^^^^ ambiguous associated type `Err`
   |
note: associated type `Self` could derive from `std::str::FromStr`
  --> src/test/compile-fail/E0221.rs:32:16
   |
32 |         let _: Self::Err;
   |                ^^^^^^^^^

