
error[E0308]: mismatched types
 --> src/main.rs:6:9
  |
6 |     c = b;
  |         ^ expected enum `std::option::Option`, found reference
  |
  = note: expected type `std::option::Option<&[u8]>`
             found type `&std::option::Option<std::vec::Vec<u8>>`
