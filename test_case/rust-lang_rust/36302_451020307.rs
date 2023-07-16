
error[E0423]: expected value, found builtin type `i32`
 --> src/main.rs:2:14
  |
2 | let x: f32 = i32; // expected type f32, found type {integer}
  |              ^^^ not a value

error[E0423]: expected value, found builtin type `i32`
 --> src/main.rs:3:18
  |
3 | let ref x: f32 = i32; // expected type {integer}, found type f32
  |                  ^^^ not a value
