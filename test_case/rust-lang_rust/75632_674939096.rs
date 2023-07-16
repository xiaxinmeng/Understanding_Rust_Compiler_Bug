
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
2 | /     match maybe_bool {
3 | |         Some(a_bool) => a_bool,
4 | |         None => true
5 | |     } && bool_slice[0]
  | |     ^- help: consider using a semicolon here
  | |_____|
  |       expected `()`, found `bool`

error[E0308]: mismatched types
 --> src/main.rs:5:7
  |
1 | fn test_match_bool(bool_slice: &[bool], maybe_bool: Option<bool>) -> bool {
  |                                                                      ---- expected `bool` because of return type
...
5 |     } && bool_slice[0]
  |       ^^^^^^^^^^^^^^^^ expected `bool`, found `&&bool`
  |
help: parentheses are required to parse this as an expression
  |
2 |     (match maybe_bool {
3 |         Some(a_bool) => a_bool,
4 |         None => true
5 |     }) && bool_slice[0]
  |
