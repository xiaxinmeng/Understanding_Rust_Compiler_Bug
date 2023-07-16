
error[E0133]: cast of pointer to int is unsafe and requires unsafe function or block
 --> src/lib.rs:3:18
  |
3 | const A: usize = &42 as *const i32 as usize;
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^ cast of pointer to int
  |
  = note: casting pointers to integers in constants
