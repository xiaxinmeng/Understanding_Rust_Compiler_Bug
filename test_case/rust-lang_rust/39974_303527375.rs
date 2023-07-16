
rustc 1.19.0-dev (5b13bff52 2017-05-23)
error[E0308]: mismatched types
 --> test.rs:8:24
  |
8 |     let t = Thing { f: [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]] };
  |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 2 elements, found one with 4 elements
  |
  = note: expected type `[[f64; 2]; 2]`
             found type `[[f64; 2]; 4]`

error: aborting due to previous error
