
error[E0121]: the type placeholder `_` is not allowed within types on item signatures
 --> src/lib.rs:1:25
  |
1 | fn returns_closure() -> _ {
  |                         ^
  |                         |
  |                         not allowed in type signatures
  |                         help: replace with the correct return type: `[closure@src/lib.rs:2:5: 2:9]`
