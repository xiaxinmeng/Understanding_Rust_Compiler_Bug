
error[E0308]: mismatched types
 --> <assert_eq macros>:5:22
  |
5 | if ! ( * left_val == * right_val ) {
  |                      ^^^^^^^^^^^ expected u64, found usize
 ::: file.rs
  |
2 |     assert_eq!(10u64, 10usize);
  |     --------------------------- in this expansion of `assert_eq!`

error: aborting due to previous error
