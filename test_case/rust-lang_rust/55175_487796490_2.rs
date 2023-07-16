
error[E0308]: mismatched types
 --> src/main.rs:4:16
  |
4 |         [v] => v,
  |                ^
  |                |
  |                expected i32, found &i32
  |                help: consider dereferencing the borrow: `*v`
  |
  = note: expected type `i32`
             found type `&i32`
