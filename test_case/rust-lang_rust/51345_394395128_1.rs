
 Compiling playground v0.0.1 (file:///playground)
warning: unreachable expression
 --> src/main.rs:6:12
  |
6 |     loop { v.push(break) }
  |            ^^^^^^^^^^^^^
  |
  = note: #[warn(unreachable_code)] on by default

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some((_3, bw0))`,
 right: `None`: never found an activation for this borrow!', librustc_mir/borrow_check/borrow_set.rs:121:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic
