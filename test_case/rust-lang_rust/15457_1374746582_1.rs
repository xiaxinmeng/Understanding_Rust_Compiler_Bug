
error[E0507]: cannot move out of `self.option` which is behind a shared reference
 --> src/main.rs:7:9
  |
7 |         self.option.map(|x| x)
  |         ^^^^^^^^^^^ ---------- `self.option` moved due to this method call
  |         |
  |         help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
  |         move occurs because `self.option` has type `Option<Vec<u8>>`, which does not implement the `Copy` trait
  |
note: `Option::<T>::map` takes ownership of the receiver `self`, which moves `self.option`
 --> /rustc/ee0412d1ef81efcfabe7f66cd21476ca85d618b1/library/core/src/option.rs:963:28
help: you can `clone` the value and consume it, but this might not be your desired behavior
  |
7 |         self.option.clone().map(|x| x)
  |                     ++++++++
