
error[E0308]: mismatched types
 --> src/main.rs:3:10
  |
3 |     test(array.len());
  |          ^^^^^^^^^^^ expected u32, found usize
  |
  = help: you can cast an `usize` to `u32`:
  |
3 |     test(array.len() as u32);
  |          ^^^^^^^^^^^^^^^^^^
  = note: if the value doesn't fit it will be truncated when casting from `usize` to `u32`
