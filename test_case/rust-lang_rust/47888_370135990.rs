rust
warning: unused imports: `io`, `net`
 --> src/main.rs:1:22
  |
1 | use std::os::{unix::{io, net}, linux};
  |                      ^^  ^^^
  |
  = note: #[warn(unused_imports)] on by default
