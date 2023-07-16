
error[E0308]: mismatched types
 --> <assert_eq macros>:5:22
  |
5 | if ! ( * left_val == * right_val ) {
  |                      ^^^^^^^^^^^ expected u64, found usize
file2.rs:2:5: 2:32 note: in this expansion of assert_eq! (defined in <assert_eq macros>)

error: aborting due to previous error
