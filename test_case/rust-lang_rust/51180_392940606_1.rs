
$ cargo +nightly build
   Compiling rand v0.5.0 (file:///home/tobias/dev/rand)
error[E0463]: can't find crate for `distributions`
   --> src/distributions/uniform.rs:104:5
    |
104 | use distributions::Distribution;
    |     ^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error
