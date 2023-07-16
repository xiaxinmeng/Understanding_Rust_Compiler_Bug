
$ rustc +A src/test/incremental/delayed_span_bug.rs
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
 --> src/test/incremental/delayed_span_bug.rs:8:1
  |
8 | fn main() {}
  | ^^^^^^^^^
  |
  = note: delayed at compiler/rustc_middle/src/util/bug.rs:45:14

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1205:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
