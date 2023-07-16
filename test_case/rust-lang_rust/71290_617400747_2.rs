
error: expected expression, found `+`
 --> src/lib.rs:2:21
  |
2 |     { u8::from(a) } + { u8::from(b) }
  |     --------------- ^ expected expression
  |     |
  |     help: parentheses are required to parse this as an expression: `({ u8::from(a) })`

error[E0308]: mismatched types
 --> src/lib.rs:2:7
  |
2 |     { u8::from(a) } + { u8::from(b) }
  |       ^^^^^^^^^^^- help: try adding a semicolon: `;`
  |       |
  |       expected `()`, found `u8`

error: aborting due to 2 previous errors
