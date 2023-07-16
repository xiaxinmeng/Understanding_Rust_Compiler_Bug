
% rustc file.rs -Z external-macro-backtrace                                                                                                                                                                        
error[E0308]: mismatched types
 --> <assert_eq macros>:5:22
  |
5 | if ! ( * left_val == * right_val ) {
  |                      ^^^^^^^^^^^ expected u8, found u16
file.rs:11:1: 11:13 note: in this expansion of my_macro! (defined in file.rs)
file.rs:6:13: 6:35 note: in this expansion of assert_eq! (defined in <assert_eq macros>)

error: aborting due to previous error
