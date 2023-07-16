text
error[[E0507]](https://doc.rust-lang.org/stable/error-index.html#E0507): cannot move out of `*vec` which is behind a shared reference
 --> src/main.rs:4:23
  |
4 |         .filter(|vec| vec.unwrap().len() > 5)
  |                       ^^^^--------
  |                       |   |
  |                       |   `*vec` moved due to this method call
  |                       move occurs because `*vec` has type `Result<Vec<i32>, ()>`, which does not implement the `Copy` trait
  |
note: this function takes ownership of the receiver `self`, which moves `*vec`
help: consider calling `.as_ref()` to borrow the type's contents
  |
4 |         .filter(|vec| vec.as_ref().unwrap().len() > 5)
  |                           +++++++++

For more information about this error, try `rustc --explain E0507`.
