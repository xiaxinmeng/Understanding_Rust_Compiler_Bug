
error[E0507]: cannot move out of `*ref_result` which is behind a shared reference
 --> src/main.rs:4:13
  |
4 |     let x = ref_result.unwrap();
  |             ^^^^^^^^^^^--------
  |             |          |
  |             |          `*ref_result` moved due to this method call
  |             move occurs because `*ref_result` has type `Result<String, String>`, which does not implement the `Copy` trait
  |
note: this function takes ownership of the receiver `self`, which moves `*ref_result`
help: consider calling `.as_ref()` to borrow the type's contents
  |
4 |     let x = ref_result.as_ref().unwrap();
  |                        +++++++++
