
error[E0495]: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
 --> src/main.rs:2:5
  |
2 |     &*x
  |     ^^^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the function body at 1:1...
 --> src/main.rs:1:1
  |
1 | / fn foo(x: &u32) -> &'static u32 {
2 | |     &*x
3 | | }
  | |_^
note: ...so that reference does not outlive borrowed content
 --> src/main.rs:2:5
  |
2 |     &*x
  |     ^^^
  = note: but, the lifetime must be valid for the static lifetime...
note: ...so that reference does not outlive borrowed content
 --> src/main.rs:2:5
  |
2 |     &*x
  |     ^^^
