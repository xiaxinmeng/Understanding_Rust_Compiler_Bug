
error: expected `;`, found `3_i8`
 --> src/lib.rs:2:10
  |
2 |     2_u32
  |          ^ help: add `;` here
3 |     3_i8
  |     ---- unexpected token

error: expected `;`, found `5.0`
 --> src/lib.rs:3:9
  |
3 |     3_i8
  |         ^ help: add `;` here
4 |     5.0
  |     --- unexpected token

error[E0308]: mismatched types
 --> src/lib.rs:2:5
  |
2 |     2_u32
  |     ^^^^^ expected `()`, found `u32`

error[E0308]: mismatched types
 --> src/lib.rs:3:5
  |
3 |     3_i8
  |     ^^^^ expected `()`, found `i8`
