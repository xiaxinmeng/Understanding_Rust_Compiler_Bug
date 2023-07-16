
error[E0308]: mismatched types
 --> src/main.rs:7:10
  |
6 |     let f = match (x, y) {
  |                   ------ this expression has type `(std::boxed::Box<S>, std::boxed::Box<S>)`
7 |         (S(a), S(b)) => (a,b),
  |          ^^^^ expected struct `std::boxed::Box`, found struct `S`
  |
  = note: expected struct `std::boxed::Box<S>`
             found struct `S`

error[E0308]: mismatched types
 --> src/main.rs:7:16
  |
6 |     let f = match (x, y) {
  |                   ------ this expression has type `(std::boxed::Box<S>, std::boxed::Box<S>)`
7 |         (S(a), S(b)) => (a,b),
  |                ^^^^ expected struct `std::boxed::Box`, found struct `S`
  |
  = note: expected struct `std::boxed::Box<S>`
             found struct `S`

error: aborting due to 2 previous errors
