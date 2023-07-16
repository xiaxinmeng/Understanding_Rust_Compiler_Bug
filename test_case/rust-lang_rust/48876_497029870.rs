
error[E0308]: mismatched types
 --> src/main.rs:6:9
  |
6 |     r = v.to_owned();
  |         ^^^^^^^^^^^^
  |         |
  |         expected struct `std::vec::Vec`, found reference
  |         help: try using a conversion method: `v.to_owned().to_vec()`
  |
  = note: expected type `std::vec::Vec<_>`
             found type `&std::vec::Vec<{integer}>`
