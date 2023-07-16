rust
error[E0308]: no return along this path
 --> src/main.rs:3:20
  |
3 |         if false { break; }
  |                    ^^^^^ following this path, no value is ever returned
  |
