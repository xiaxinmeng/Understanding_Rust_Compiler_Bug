
warning: unreachable call
 --> src/lib.rs:2:5
  |
2 |     foo(
  |     ^^^ unreachable call
3 |         panic!()
  |         -------- any code following this expression is unreachable
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
