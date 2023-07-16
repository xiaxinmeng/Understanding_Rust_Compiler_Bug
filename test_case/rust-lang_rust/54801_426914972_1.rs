
error: expected pattern, found `#`
 --> src/lib.rs:3:8
  |
L | fn foo(#[allow(dead_code)] id: i32) {}
  |        ^ expected pattern

error: expected type, found `#`
 --> src/lib.rs:5:12
  |
L | fn bar(id: #[allow(dead_code)] i32) {}
  |            ^
