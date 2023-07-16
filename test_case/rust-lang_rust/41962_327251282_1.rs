
error[E0382]: use of partially moved value: `maybe`
 --> file.rs:5:30
  |
5 |         if let Some(thing) = maybe {
  |                     -----    ^^^^^ value used here after move
  |                     |
  |                     value moved here
  |
  = note: move occurs because `(maybe:std::prelude::v1::Some).0` has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `(maybe:std::prelude::v1::Some).0`
 --> file.rs:5:21
  |
5 |         if let Some(thing) = maybe {
  |                     ^^^^^ value moved here in previous iteration of loop
  |
  = note: move occurs because `(maybe:std::prelude::v1::Some).0` has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait

error: aborting due to previous error(s)
