
error: missing documentation for the crate
 --> src/lib.rs:1:1
  |
1 | / //! Doc this
2 | | #![cfg(windows)]
3 | |
4 | | /// Doc this
5 | | pub fn a_function() {}
  | |______________________^
  |
  = note: requested on the command line with `-D missing-docs`

error: aborting due to previous error
