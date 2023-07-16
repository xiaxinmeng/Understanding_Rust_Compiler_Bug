 rust
   Compiling playground v0.0.1 (/playground)
warning: missing documentation for the crate
 --> src/lib.rs:1:1
  |
1 | / #![feature(trait_alias)]
2 | | #![warn(missing_docs)]
3 | |
4 | | pub trait T = Send;
  | |___________________^
  |
note: the lint level is defined here
 --> src/lib.rs:2:9
  |
2 | #![warn(missing_docs)]
  |         ^^^^^^^^^^^^
  