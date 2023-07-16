
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
 --> src/main.rs:3:1
  |
3 | impl From<i32> for Result<(), MyError> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------^ impl doesn't use types inside crate
  |                               |
  |                               even though this type is defined here, `Result` isn't
  |
  = note: the impl does not reference only types defined in this crate
  = note: define and implement a trait or new type instead
