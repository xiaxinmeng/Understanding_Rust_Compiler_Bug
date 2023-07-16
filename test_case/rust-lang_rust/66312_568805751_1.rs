
error[E0277]: the trait bound `std::option::Option<{integer}>: std::ops::Deref` is not satisfied
 --> src/lib.rs:8:10
  |
8 |     if x.is_some() {
  |          ^^^^^^^ the trait `std::ops::Deref` is not implemented for `std::option::Option<{integer}>`

error[E0308]: mismatched types
 --> src/lib.rs:8:8
  |
8 |     if x.is_some() {
  |        ^^^^^^^^^^^ expected `bool`, found `()`

error: aborting due to 2 previous errors

