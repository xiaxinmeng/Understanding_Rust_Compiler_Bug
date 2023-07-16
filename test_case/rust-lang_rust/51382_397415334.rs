
error: missing documentation for a type alias
  --> libstd\sys\unix/ext\raw.rs:21:49
   |
21 | #[stable(feature = "raw_ext", since = "1.1.0")] pub type uid_t = u32;
   |                                                 ^^^^^^^^^^^^^^^^^^^^^
   |
note: lint level defined here
  --> libstd\lib.rs:224:9
   |
224| #![deny(missing_docs)]
   |         ^^^^^^^^^^^^
