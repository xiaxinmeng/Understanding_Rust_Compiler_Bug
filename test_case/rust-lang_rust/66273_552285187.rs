
error[E0005]: refutable pattern in function argument: `&_` not covered
 --> src/main.rs:3:13
  |
1 | const value: &str = "";
  | ----------------------- constant defined here
2 | 
3 | fn to_owned(value: &str) -> String {
  |             ^^^^^
  |             |
  |             interpreted as a constant pattern, not a new variable
  |             help: introduce a variable instead: `value_var`
