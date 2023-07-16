
error[E0308]: mismatched types
  --> src/main.rs:24:10
   |
24 |     mask(1..23)
   |     ---- ^^^^^ expected struct `Once`, found struct `Range`
   |     |
   |     arguments to this function are incorrect
   |
   = note: expected struct `std::iter::Once<i32>`
              found struct `std::ops::Range<{integer}>`
note: function defined here
  --> src/main.rs:5:4
   |
5  | fn mask<T: Iterator<Item = i32>>(iter: T) -> impl Iterator<Item = i32> {
   |    ^^^^                          -------
