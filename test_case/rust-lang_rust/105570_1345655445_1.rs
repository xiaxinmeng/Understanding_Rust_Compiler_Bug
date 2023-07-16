
error: no rules expected the token `false`
 --> src/main.rs:9:11
  |
1 | macro_rules! number {
  | ------------------- when calling this macro
...
9 | number! { false => u8; }
  |           ^^^^^ no rules expected this token in macro call
  |
note: while trying to match `=>`
 --> src/main.rs:3:17
  |
3 |     ($signed:tt => $ty:ty;) => {
  |                 ^^
