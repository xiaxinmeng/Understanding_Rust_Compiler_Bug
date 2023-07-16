
error[E0308]: mismatched types
 --> src/main.rs:3:9
  |
3 |     foo(a);
  |         ^
  |         |
  |         expected &[u8], found struct `std::vec::Vec`
  |         help: consider borrowing here: `&a`
  |
  = note: expected type `&[u8]`
             found type `std::vec::Vec<_>`
