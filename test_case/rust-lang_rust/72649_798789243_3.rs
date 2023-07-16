rust
error[E0381]: use of possibly-uninitialized variable: `value`
 --> src/lib.rs:4:16
  |
4 |     let used = value;
  |                ^^^^^ use of possibly-uninitialized `value`
