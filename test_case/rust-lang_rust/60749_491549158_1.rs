
error: literal out of range for `u8`
 --> src/lib.rs:4:8
  |
4 | impl S<256> { fn foo() {} }
  |        ^^^
  |
  = note: #[deny(overflowing_literals)] on by default
