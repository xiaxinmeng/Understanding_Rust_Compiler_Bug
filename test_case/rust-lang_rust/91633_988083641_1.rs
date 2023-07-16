
$ rustc +1.41.0 issue-91633.rs 
error: internal compiler error: broken MIR in DefId(0:10 ~ issue_91633[317d]::{{impl}}[0]::index[0]) (_3 = &(*_1)[_4]): bad assignment (&<[T] as std::ops::Index<usize>>::Output = &T): NoSolution
 --> issue-91633.rs:9:9
  |
9 |         &self[index.0 as usize]
  |         ^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:359:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0 (5e1a79984 2020-01-27) running on x86_64-unknown-linux-gnu

