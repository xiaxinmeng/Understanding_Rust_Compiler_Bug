
warning: unnecessary `unsafe` block
 --> src/main.rs:2:5
  |
1 | unsafe fn foo() -> u32 {
  | ---------------------- because it's nested under this `unsafe` fn
2 |     unsafe {
  |     ^^^^^^ unnecessary `unsafe` block
  |
  = note: `#[warn(unused_unsafe)]` on by default
  = note: this `unsafe` block does contain unsafe operations, but those are already allowed in an `unsafe fn`
  = note: `#[allow(unsafe_op_in_unsafe_fn)]` on by default
