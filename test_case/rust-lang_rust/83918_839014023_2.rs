
error[E0527]: pattern requires 2 elements but array has 8
 --> src/main.rs:6:5
  |
6 | let [first_two @ ..2, rest @ 2..] = xs;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 8 elements

error: aborting due to previous error
