
error: constant expression depends on a generic parameter
 --> src/lib.rs:3:19
  |
3 |     let x = [0u8; std::mem::size_of::<T>()];
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes

error: constant expression depends on a generic parameter
 --> src/lib.rs:5:5
  |
5 |     std::mem::transmute(x)
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes
