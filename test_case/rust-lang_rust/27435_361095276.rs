
error[E0121]: the type placeholder `_` is not allowed within types on item signatures
 --> src/main.rs:1:22
  |
1 | fn shuffle(args: Vec<_>) -> Vec<_> {}
  |                      ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
 --> src/main.rs:1:33
  |
1 | fn shuffle(args: Vec<_>) -> Vec<_> {}
  |                                 ^ not allowed in type signatures
