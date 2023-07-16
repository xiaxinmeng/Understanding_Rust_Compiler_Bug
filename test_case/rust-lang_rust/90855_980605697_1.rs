
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
 --> src/main.rs:2:18
  |
2 | const Y: usize = S.len();
  |                  ^^^^^^^

error[E0493]: destructors cannot be evaluated at compile-time
 --> src/main.rs:2:18
  |
2 | const Y: usize = S.len();
  |                  ^     - value is dropped here
  |                  |
  |                  constants cannot evaluate destructors
