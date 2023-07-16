
error[E0308]: mismatched types
 --> src/main.rs:3:10
  |
3 |     test(array.len());
  |          ^^^^^^^^^^^ expected u32, found usize
help: you can convert an `usize` to `u32` or panic if it the converted value wouldn't fit
  |
3 |     test(array.len().try_into().unwrap());
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
