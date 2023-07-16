
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> issue-80059.rs:2:12
  |
2 |     let _: bool = *ptr;
  |            ^^^^ dereference of raw pointer
  |
  = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
