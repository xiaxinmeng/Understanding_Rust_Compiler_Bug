rust
error: any use of this value will cause an error
 --> src/lib.rs:2:5
  |
2 |     arg + i32::MAX
  |     ^^^^^^^^^^^^^^
  |     |
  |     attempt to add with overflow
  |     inside `foo` at src/lib.rs:2:5
  |     inside `x` at src/lib.rs:4:16
3 | }
4 | const x: i32 = foo(1);
  | ----------------------
  |
  = note: `#[deny(const_err)]` on by default
