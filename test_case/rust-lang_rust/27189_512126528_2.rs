
error: argument never used
 --> /home/user/build/2nonpkgs/rust.stuff/plaincatchup/bad_Z/src/main.rs:7:9
  |
3 |         "Hello, world! {}",
  |         ------------------ formatting specifier missing
...
7 |         "arg when bad",
  |         ^^^^^^^^^^^^^^ argument never used

thread '<unnamed>' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:546:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-dev (e7efdf1c3 2019-07-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=1 -C opt-level=3 -C lto -C codegen-units=1 -C debug-assertions=on -C target-cpu=native --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `bad_Z`.
