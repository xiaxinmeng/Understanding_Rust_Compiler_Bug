
$ echo I1vQhQ== | base64 -D > main.rs

$ rustc main.rs
error: this file contains an un-closed delimiter
 --> main.rs:1:4
  |
1 | #[Ѕ
  |  - ^
  |  |
  |  un-closed delimiter

thread 'rustc' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', src/libsyntax/source_map.rs:824:17
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0 (a53f9df32 2019-07-03) running on x86_64-apple-darwin
