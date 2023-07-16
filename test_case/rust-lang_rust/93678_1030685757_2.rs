
error: call to unsafe function is unsafe and requires unsafe block (error E0133)
  --> src/main.rs:13:13
   |
13 |             unsf();
   |             ^^^^^^ call to unsafe function
   |
note: the lint level is defined here
  --> src/main.rs:11:16
   |
11 |         #[deny(unsafe_op_in_unsafe_fn)]
   |                ^^^^^^^^^^^^^^^^^^^^^^
   = note: consult the function's documentation for information on how to avoid undefined behavior

warning: unnecessary `unsafe` block
  --> src/main.rs:10:5
   |
9  | unsafe fn granular_disallow_op_in_unsafe_fn() {
   | --------------------------------------------- because it's nested under this `unsafe` fn
10 |     unsafe {
   |     ^^^^^^ unnecessary `unsafe` block
   |
   = note: `#[warn(unused_unsafe)]` on by default

