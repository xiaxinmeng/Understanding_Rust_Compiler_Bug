rust
error[E0382]: use of partially moved value: `maybe`
 --> test.rs:7:30
  |
7 |         if let Some(thing) = maybe {
  |                     -----    ^^^^^ value used here after move
  |                     |
  |                     value moved here, because it has type `std::vec::Vec<bool>`, which does't implement the `Copy` trait

error: aborting due to previous error
