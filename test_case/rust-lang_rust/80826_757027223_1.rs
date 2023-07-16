
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> <anon>:2:18
  |
2 |     let _: bool = { *ptr };
  |                     ^^^^ dereference of raw pointer
  |
  = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
