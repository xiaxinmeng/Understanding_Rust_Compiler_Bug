
dswafford@Davids-Mac-mini.local:/Volumes/dswafford/CODE/rust/hello/grrs$cargo run --verbose
   Compiling grrs v0.1.0 (/Volumes/dswafford/CODE/rust/hello/grrs)
     Running `rustc --crate-name grrs --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=a4d4421a81652fc4 -C extra-filename=-a4d4421a81652fc4 --out-dir /Volumes/dswafford/CODE/rust/hello/grrs/target/debug/deps -C incremental=/Volumes/dswafford/CODE/rust/hello/grrs/target/debug/incremental -L dependency=/Volumes/dswafford/CODE/rust/hello/grrs/target/debug/deps`
error: incremental compilation: could not create session directory lock file: Operation not supported (os error 45)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', src/librustc_session/session.rs:690:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0 (49cae5576 2020-06-01) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error

error: could not compile `grrs`.

Caused by:
  process didn't exit successfully: `rustc --crate-name grrs --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=a4d4421a81652fc4 -C extra-filename=-a4d4421a81652fc4 --out-dir /Volumes/dswafford/CODE/rust/hello/grrs/target/debug/deps -C incremental=/Volumes/dswafford/CODE/rust/hello/grrs/target/debug/incremental -L dependency=/Volumes/dswafford/CODE/rust/hello/grrs/target/debug/deps` (exit code: 101)
