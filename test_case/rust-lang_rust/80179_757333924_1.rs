
error[E0121]: the type placeholder `_` is not allowed within types on item signatures
 --> src/lib.rs:3:13
  |
3 | fn foo() -> _ {
  |             ^
  |             |
  |             not allowed in type signatures
  |             help: replace with the correct return type: [generator@src/lib.rs:4:5: 4:18 {i32, ()}]
