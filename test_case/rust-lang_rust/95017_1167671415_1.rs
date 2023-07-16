console
$ cargo +1.61.0 check
    Checking repro v0.0.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s

$ cargo +1.60.0 check
    Checking repro v0.0.0

error: to use a constant of type `std::cmp::Ordering` in a pattern, `std::cmp::Ordering` must be annotated with `#[derive(PartialEq, Eq)]`
 --> src/main.rs:3:12
  |
3 |     if let ORDERING = ORDERING {}
  |            ^^^^^^^^

warning: irrefutable `if let` pattern
 --> src/main.rs:3:8
  |
3 |     if let ORDERING = ORDERING {}
  |        ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(irrefutable_let_patterns)]` on by default
  = note: this pattern will always match, so the `if let` is useless
  = help: consider replacing the `if let` with a `let`

error: could not compile `repro` due to previous error; 1 warning emitted
