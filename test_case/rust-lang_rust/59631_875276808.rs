
error[E0391]: cycle detected when computing type of `FOO::{opaque#0}`
 --> src/main.rs:2:13
  |
2 | static FOO: impl Copy = &FOO;
  |             ^^^^^^^^^
  |
note: ...which requires borrow-checking `FOO`...
 --> src/main.rs:2:1
  |
2 | static FOO: impl Copy = &FOO;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `FOO`...
 --> src/main.rs:2:1
  |
2 | static FOO: impl Copy = &FOO;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `FOO`...
 --> src/main.rs:2:1
  |
2 | static FOO: impl Copy = &FOO;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
  = note: ...which requires evaluating trait selection obligation `impl std::marker::Copy: std::marker::Freeze`...
  = note: ...which again requires computing type of `FOO::{opaque#0}`, completing the cycle
  = note: cycle used when normalizing `impl std::marker::Copy`
