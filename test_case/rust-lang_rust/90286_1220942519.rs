
error[E0507]: cannot move out of `*opt` which is behind a shared reference
 --> src/lib.rs:2:5
  |
2 |     opt.map(|x| x.to_string()).unwrap_or_else(String::new)
  |     ^^^^----------------------
  |     |   |
  |     |   `*opt` moved due to this method call
  |     move occurs because `*opt` has type `Option<Box<i32>>`, which does not implement the `Copy` trait
  |
note: this function takes ownership of the receiver `self`, which moves `*opt`
help: consider calling `.as_ref()` to borrow the type's contents
  |
2 |     opt.as_ref().map(|x| x.to_string()).unwrap_or_else(String::new)
  |         +++++++++
