text
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> src/lib.rs:4:12
  |
4 |    let _ = &*input;
  |            ^^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
