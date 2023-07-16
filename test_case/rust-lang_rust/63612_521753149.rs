
error[E0308]: mismatched types
 --> src/test/ui/suggestions/dont-suggest-try_into-in-macros.rs:2:5
  |
2 |     assert_eq!(10u64, 10usize); //~ ERROR mismatched types
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u64, found usize
  |
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
help: you can convert an `usize` to `u64` and panic if the converted value wouldn't fit
  |
7 |              if ! (* left_val == (* right_val).try_into().unwrap())
  |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
