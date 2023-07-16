
error: expected pattern, found `#`
 --> src/lib.rs:3:8
  |
3 | fn foo(#[allow(dead_code)] id: i32) {}
  |        ^ attributes are not allowed on function parameters.

error: expected type, found `#`
 --> src/lib.rs:5:12
  |
5 | fn bar(id: #[allow(dead_code)] i32) {}
  |            ^ attributes are not allowed on function parameter types.
