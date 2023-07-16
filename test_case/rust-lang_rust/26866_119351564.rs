
[andrew@Liger qc] cargo run --verbose
   Compiling qc v0.1.0 (file:///tmp/qc)
     Running `rustc src/main.rs --crate-name qc --crate-type bin -g --out-dir /tmp/qc/target/debug --emit=dep-info,link -L dependency=/tmp/qc/target/debug -L dependency=/tmp/qc/target/debug/deps`
../src/libcore/iter.rs:804:35: 804:42 error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<Foo as core::cmp::PartialOrd>)` during trans
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/diagnostic.rs:176


Could not compile `qc`.

Caused by:                                                                                                        
  Process didn't exit successfully: `rustc src/main.rs --crate-name qc --crate-type bin -g --out-dir /tmp/qc/target/debug --emit=dep-info,link -L dependency=/tmp/qc/target/debug -L dependency=/tmp/qc/target/debug/deps` (exit code: 101)
