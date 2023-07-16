
error: any use of this value will cause an error
 --> src/main.rs:3:22
  |
3 | const TEST_2: bool = TEST;
  | ---------------------^^^^-
  |                      |
  |                      referenced constant has errors
  |
  = note: `#[deny(const_err)]` on by default
