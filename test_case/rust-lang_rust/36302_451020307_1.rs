
error[E0308]: mismatched types
 --> src/main.rs:2:14
  |
2 | let x: f32 = 3; // expected type f32, found type {integer}
  |              ^
  |              |
  |              expected f32, found integral variable
  |              help: use a float literal: `3.0`
  |
  = note: expected type `f32`
             found type `{integer}`

error[E0308]: mismatched types
 --> src/main.rs:3:18
  |
3 | let ref x: f32 = 3; // expected type {integer}, found type f32
  |                  ^
  |                  |
  |                  expected f32, found integral variable
  |                  help: use a float literal: `3.0`
  |
  = note: expected type `f32`
             found type `{integer}`
