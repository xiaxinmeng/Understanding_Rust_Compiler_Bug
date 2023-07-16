rust
error[E0493]: destructor of `[T; 0]` cannot be evaluated at compile-time
 --> src/main.rs:2:6
  |
2 |     &[]
  |      ^^ the destructor for this type cannot be evaluated in constant functions
3 | }
  | - value is dropped here
