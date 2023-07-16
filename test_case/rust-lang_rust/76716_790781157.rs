
warning: missing documentation for the crate
 --> hidden.rs:1:1
  |
1 | / #![warn(missing_docs)]
2 | | #[doc(hidden)]
3 | | pub fn f() {}
  | |_____________^
  |
note: the lint level is defined here
 --> hidden.rs:1:9
  |
1 | #![warn(missing_docs)]
  |         ^^^^^^^^^^^^
