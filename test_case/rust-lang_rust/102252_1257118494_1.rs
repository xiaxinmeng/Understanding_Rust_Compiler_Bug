
   Compiling ice v0.0.0 (/home/ubuntu/dev/ice)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src/lib.rs:1:1
  |
1 | #![feature(min_specialization, rustc_attrs)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'region constraints already solved', src/librustc_infer/infer/mod.rs:209:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.1 (c7087fe00 2020-06-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error

For more information about this error, try `rustc --explain E0554`.
error: could not compile `ice`.

To learn more, run the command again with --verbose.
