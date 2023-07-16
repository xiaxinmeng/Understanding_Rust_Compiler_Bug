
error[E0518]: attribute should be applied to function or closure
 --> src/main.rs:6:5
  |
6 |     #[inline(always)] async { 12345 }
  |     ^^^^^^^^^^^^^^^^^ --------------- not a function or closure
