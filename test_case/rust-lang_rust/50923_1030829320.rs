rust
error[E0425]: cannot find value `PrivateMember` in this scope
 --> src/main.rs:6:5
  |
6 |     PrivateMember;
  |     ^^^^^^^^^^^^^ not found in this scope
  |
note: unit struct `crate::other_module::PrivateMember` exists but is inaccessible
--> src/main.rs:2:5
  |
2 |     struct PrivateMember;
  |     ^^^^^^^^^^^^^^^^^^^^^ not accessible
