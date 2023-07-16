
$ cargo +nightly check
error: internal compiler error: bad placeholder type
 --> src/lib.rs:4:21
  |
4 |                     _, diesel::query_source::joins::Inner, _
  |                     ^

error: internal compiler error: bad placeholder type
 --> src/lib.rs:4:60
  |
4 |                     _, diesel::query_source::joins::Inner, _
  |                                                            ^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:347:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (859764425 2020-01-07) running on x86_64-unknown-linux-musl

note: compiler flags: -C debuginfo=2 -C incremental -C target-feature=-crt-static --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: could not compile `vinoteca`.
