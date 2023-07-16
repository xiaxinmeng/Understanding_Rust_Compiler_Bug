
error[E0391]: cycle detected when computing type of `foo::{{opaque}}#0`
 --> src/main.rs:3:16
  |
3 | async fn foo() {
  |                ^
  |
note: ...which requires borrow-checking `foo`...
 --> src/main.rs:3:1
  |
3 | async fn foo() {
  | ^^^^^^^^^^^^^^
note: ...which requires processing `foo`...
 --> src/main.rs:3:1
  |
3 | async fn foo() {
  | ^^^^^^^^^^^^^^
note: ...which requires processing MIR for `foo`...
 --> src/main.rs:3:1
  |
3 | async fn foo() {
  | ^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `foo`...
 --> src/main.rs:3:1
  |
3 | async fn foo() {
  | ^^^^^^^^^^^^^^
note: ...which requires building MIR for `foo`...
 --> src/main.rs:3:1
  |
3 | async fn foo() {
  | ^^^^^^^^^^^^^^
note: ...which requires type-checking `foo`...
 --> src/main.rs:3:1
  |
3 | async fn foo() {
  | ^^^^^^^^^^^^^^
  = note: ...which requires evaluating trait selection obligation `{std::future::ResumeTy, impl futures::Future, ()}: std::marker::Send`...
  = note: ...which again requires computing type of `foo::{{opaque}}#0`, completing the cycle
note: cycle used when checking item types in top-level module
 --> src/main.rs:1:1
  |
1 | / use futures::future::FutureExt;
2 | |
3 | | async fn foo() {
4 | |     FutureExt::boxed(async {
... |
8 | |
9 | | fn main() {}
  | |____________^

error: aborting due to previous error
