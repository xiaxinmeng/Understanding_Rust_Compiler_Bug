rust
warning: struct is never used: `S`
 --> src/main.rs:8:10
  |
3 |         $b $a;
  |         ^^^^^^ unused struct
...
8 |     m!(S struct);
  |     ------------- in this macro invocation
  |
  = note: #[warn(dead_code)] on by default
