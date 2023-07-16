rust
warning: unreachable statement
 --> issue-78333.rs:5:5
  |
4 |     ::std::todo!();
  |     --------------- any code following this expression is unreachable
5 |     ::std::unimplemented!();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 1 warning emitted
