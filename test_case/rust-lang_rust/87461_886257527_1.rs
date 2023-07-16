
error[E0308]: mismatched types
 --> test.rs:6:12
  |
6 |         Ok(())
  |            ^^ expected `u16`, found `()`
  |
note: return type inferred to be `u16` here
 --> test.rs:4:20
  |
4 |             return Err(42u64);
  |                    ^^^^^^^^^^

error: aborting due to previous error
