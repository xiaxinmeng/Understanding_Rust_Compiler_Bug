
error: constant expression depends on a generic parameter
 --> src/lib.rs:5:17
  |
5 |     fn foo() -> [u8; Self::N];
  |                 ^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes
