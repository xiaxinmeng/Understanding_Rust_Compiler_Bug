
error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
 --> main.rs:3:27
  |
3 |     fn a(aa: B) -> Result<_, B> {
  |                           ^ not allowed in type signatures

error[E0391]: cycle detected when computing function signature of `A::a`
 --> main.rs:3:5
  |
3 |     fn a(aa: B) -> Result<_, B> {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: ...which requires type-checking `A::a`...
 --> main.rs:3:5
  |
3 |     fn a(aa: B) -> Result<_, B> {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires computing the variances of `B`...
 --> main.rs:8:1
  |
8 | enum B {}
  | ^^^^^^
  = note: ...which requires computing the variances for items in this crate...
  = note: ...which again requires computing function signature of `A::a`, completing the cycle
note: cycle used when collecting item types in top-level module
 --> main.rs:1:1
  |
1 | fn main() {}
  | ^^^^^^^^^

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0121, E0391.
For more information about an error, try `rustc --explain E0121`.
