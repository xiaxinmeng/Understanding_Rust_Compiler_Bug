
error: constant pattern depends on a generic parameter
  --> src/lib.rs:10:16
   |
10 |         if let Self::INITIAL_VALUE = *self {
   |                ^^^^^^^^^^^^^^^^^^^

warning: irrefutable `if let` pattern
  --> src/lib.rs:10:12
   |
10 |         if let Self::INITIAL_VALUE = *self {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(irrefutable_let_patterns)]` on by default
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`
