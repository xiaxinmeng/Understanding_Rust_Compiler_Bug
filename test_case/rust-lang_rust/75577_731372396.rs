
cargo +nightly run
   Compiling paste v1.0.3
   Compiling macro_testing v0.1.0 (C:\Users\user\Downloads\macro_testing)
thread 'rustc' panicked at 'unable to allocate fiber: The paging file is too small for this operation to complete. (os error 1455)', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\stacker-0.1.12\src\lib.rs:352:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (fe982319a 2020-11-19) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack

thread 'rustc' has overflowed its stack
error: could not compile `macro_testing`

Caused by:
  process didn't exit successfully: `rustc --crate-name macro_testing --edition=2018 src\main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=4f528d08987c48f9 --out-dir C:\Users\user\Downloads\macro_testing\target\debug\deps -C incremental=C:\Users\user\Downloads\macro_testing\target\debug\incremental -L dependency=C:\Users\user\Downloads\macro_testing\target\debug\deps --extern paste=C:\Users\user\Downloads\macro_testing\target\debug\deps\paste-4388445a40bd90ce.dll` (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)
