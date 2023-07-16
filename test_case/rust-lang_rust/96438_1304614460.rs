
Compiling playground v0.0.1 (/playground)
error[[E0507]](https://doc.rust-lang.org/nightly/error-index.html#E0507): cannot move out of dereference of `RefMut<'_, Option<Vec<()>>>`
 --> src/lib.rs:4:5
  |
4 |     cell.borrow_mut().unwrap().pop().unwrap();
  |     ^^^^^^^^^^^^^^^^^^--------
  |     |                 |
  |     |                 value moved due to this method call
  |     help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
  |     move occurs because value has type `Option<Vec<()>>`, which does not implement the `Copy` trait
  |
note: this function takes ownership of the receiver `self`, which moves value

For more information about this error, try `rustc --explain E0507`.
error: could not compile `playground` due to previous error
