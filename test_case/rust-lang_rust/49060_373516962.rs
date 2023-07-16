
error[E0017]: references in statics may only refer to immutable values
 --> src/main.rs:2:25
  |
2 | static A: & mut [i32] = &mut [0; 5];
  |                         ^^^^^^^^^^^ statics require immutable values

error[E0389]: cannot assign to data in a `&` reference
 --> src/main.rs:5:5
  |
5 |     A[3] = 3;
  |     ^^^^^^^^ assignment into an immutable reference
