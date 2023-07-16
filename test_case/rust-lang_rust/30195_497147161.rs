
error[E0308]: mismatched types
 --> src/main.rs:3:9
  |
3 |     foo(a);
  |         ^
  |         |
  |         expected struct `std::vec::Vec`, found &[_; 0]
  |         help: try using a conversion method: `a.to_vec()`
  |
  = note: expected type `std::vec::Vec<u8>`
             found type `&[_; 0]`
