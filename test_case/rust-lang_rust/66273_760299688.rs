
error[E0308]: mismatched types
 --> src/main.rs:3:13
  |
1 | const value: u8 = 0;
  | -------------------- constant defined here
2 | 
3 | fn to_owned(value: &str) -> String {
  |             ^^^^^  ---- expected due to this
  |             |
  |             expected `&str`, found `u8`
  |             `value` is interpreted as a constant, not a new binding
  |             help: introduce a new binding instead: `other_value`
