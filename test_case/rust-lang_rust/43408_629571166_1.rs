
error: constant expression depends on a generic parameter
 --> src/main.rs:2:11
  |
2 |     [0u8; std::mem::size_of::<&T>()];
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes
