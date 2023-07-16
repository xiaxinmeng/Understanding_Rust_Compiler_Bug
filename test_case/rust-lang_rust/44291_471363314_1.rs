text
error[E0308]: mismatched types
 --> src/lib.rs:2:3
  |
2 |   |var: &mut u32| {},
  |   ^^^^^^^^^^^^^^^^^^ expected "C" fn, found "Rust" fn
  |
  = note: expected type `for<'r> extern "C" fn(&'r mut u32)`
             found type `[closure@src/lib.rs:2:3: 2:21]`

