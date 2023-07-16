
error[E0308]: mismatched types
 --> src/main.rs:7:9
  |
7 |     c = *b;
  |         ^^ expected &[u8], found struct `std::vec::Vec`
  |
  = note: expected type `std::option::Option<&[u8]>`
             found type `std::option::Option<std::vec::Vec<u8>>`
