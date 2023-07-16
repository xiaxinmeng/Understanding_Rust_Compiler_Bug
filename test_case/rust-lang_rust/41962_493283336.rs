
error[E0382]: use of moved value
 --> src/main.rs:6:21
  |
6 |         if let Some(thing) = maybe {
  |                     ^^^^^ value moved here, in previous iteration of loop
  |
  = note: move occurs because value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
