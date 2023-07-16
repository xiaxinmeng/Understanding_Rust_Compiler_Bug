
error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
 --> t.rs:3:19
  |
3 | async fn foo() -> std::result::Result<()> {
  |                   ^^^^^^^^^^^^^^^^^^^^^^^
  |
note: ...which requires processing `foo::{{opaque}}#0`...
 --> t.rs:3:19
  |
3 | async fn foo() -> std::result::Result<()> {
  |                   ^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `foo::{{opaque}}#0`...
 --> t.rs:3:19
  |
3 | async fn foo() -> std::result::Result<()> {
  |                   ^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
note: cycle used when processing `foo`
 --> t.rs:3:1
  |
3 | async fn foo() -> std::result::Result<()> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
