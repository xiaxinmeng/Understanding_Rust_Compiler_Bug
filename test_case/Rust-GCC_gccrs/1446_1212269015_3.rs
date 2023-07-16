rust
error[E0308]: mismatched types
 --> test-rustc.rs:8:9
  |
8 |         this
  |         ^^^^ expected `()`, found `u32`
  |
help: you might have meant to return this value
  |
8 |         return this;
  |         ++++++     +
