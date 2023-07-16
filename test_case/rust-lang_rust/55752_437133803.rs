
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
 --> src/main.rs:3:1
  |
3 | impl From<i32> for Result<(), MyError> {
  | ^^^^^----^^^^^^^^^^------^^^^^^^^^^^^^ impl doesn't use types inside crate
  |      |             |
  |      |             this type is isn't defined in this crate
  |      this trait isn't defined in this crate
  |
  = note: the impl does not reference only types defined in this crate
  = note: define and implement a trait or new type instead
