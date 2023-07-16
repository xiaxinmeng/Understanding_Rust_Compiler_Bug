
error[E0277]: can't compare `[u8]` with `std::vec::Vec<u8>`
 --> src/main.rs:5:22
  |
5 |     println!("{}", a == b);
  |                      ^^ no implementation for `[u8] == std::vec::Vec<u8>`
  |
  = help: the trait `std::cmp::PartialEq<std::vec::Vec<u8>>` is not implemented for `[u8]`
  = note: required because of the requirements on the impl of `std::cmp::PartialEq<&std::vec::Vec<u8>>` for `&[u8]`
