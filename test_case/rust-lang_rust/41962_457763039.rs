
error[E0382]: use of partially moved value: `maybe`
 --> src/main.rs:6:30
  |
6 |         if let Some(thing) = maybe {
  |                     -----    ^^^^^ value used here after move
  |                     |
  |                     value moved here
  |
  = note: move occurs because the value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `(maybe as std::prelude::v1::Some).0`
 --> src/main.rs:6:21
  |
6 |         if let Some(thing) = maybe {
  |                     ^^^^^ value moved here in previous iteration of loop
  |
  = note: move occurs because the value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
