
error: constant expression depends on a generic parameter
 --> src/lib.rs:5:15
  |
5 |     fn fun(x: [u8; Self::N]);
  |               ^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes
