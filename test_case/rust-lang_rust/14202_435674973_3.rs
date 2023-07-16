
thread 'main' panicked at 'assertion failed: x < (u32::MAX as usize)', libsyntax/ast.rs:221:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (04fdb44f5 2018-11-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unpretty=flowgraph=4294967295 -Z flowgraph-print-loans -Z flowgraph-print-moves -Z flowgraph-print-assigns -Z flowgraph-print-all -Z unstable-options
