
error[E0277]: the trait bound `[u8; 1]: std::cmp::PartialEq<std::vec::Vec<u8>>` is not satisfied
 --> <std macros>:5:6
  |
5 | if ! ( * left_val == * right_val ) {
  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
foo.rs:3:3: 3:24 note: in this expansion of assert_eq! (defined in <std macros>)
  |
  = help: the following implementations were found:
  = help:   <[A; 0] as std::cmp::PartialEq<[B; 0]>>
  = help:   <[A; 0] as std::cmp::PartialEq<[B]>>
  = help:   <[B] as std::cmp::PartialEq<[A; 0]>>
  = help:   <[A; 0] as std::cmp::PartialEq<&'b [B]>>
  = help: and 228 others
  = note: required because of the requirements on the impl of `std::cmp::PartialEq<&std::vec::Vec<u8>>` for `&[u8; 1]`

error: aborting due to previous error
