
error[E0308]: mismatched types
 --> file2.rs:7:9
  |
6 |     fn method(&self) -> Option<Vec<u8>> {
  |                         --------------- expected `std::option::Option<std::vec::Vec<u8>>` because of return type
7 |         self.option.as_ref().map(|x| x)
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found reference
  |
  = note: expected type `std::option::Option<std::vec::Vec<u8>>`
             found type `std::option::Option<&std::vec::Vec<u8>>`
