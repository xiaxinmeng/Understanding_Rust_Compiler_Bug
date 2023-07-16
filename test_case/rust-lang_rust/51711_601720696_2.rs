
cargo build
   Compiling deleteme v0.1.0 (file:///Users/ryanlevick/deleteme)
error[E0391]: cycle detected when computing the supertraits of `HashDB`
 --> src/lib.rs:3:1
  |
3 | pub trait HashDB<H: Hasher>: AsHashDB<H> {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: ...which requires computing the supertraits of `AsHashDB`...
 --> src/lib.rs:4:1
  |
4 | pub trait AsHashDB<H: Hasher>: std::convert::AsRef<HashDB<H>> {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...which again requires computing the supertraits of `HashDB`, completing the cycle

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
error: Could not compile `deleteme`.

To learn more, run the command again with --verbose
