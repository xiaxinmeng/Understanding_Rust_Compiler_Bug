plain
[00:00:45] configure: rust.quiet-tests     := True
---
[00:20:29] error: unused variable: `arg_0`
[00:20:29]   --> libcore/ops/range.rs:48:38
[00:20:29]    |
[00:20:29] 48 | #[derive(Copy, Clone, PartialEq, Eq, Hash)]
[00:20:29]    |                                      ^^^^ help: consider using `_arg_0` instead
---
[00:20:29]    = note: #[deny(unused_variables)] implied by #[deny(warnings)]
[00:20:29]
[00:20:29] error: unused variable: `arg_0`
[00:20:29]     --> libcore/option.rs:1219:62
[00:20:29]      |
[00:20:29] 1219 | #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
[00:20:29]      |                                                              ^^^^ help: consider using `_arg_0` insteaderror: aborting due to 3 previous errors
[00:20:29]
[00:20:29] error: Could not compile `core`.
[00:20:29]
[00:20:29] Caused by:
[00:20:29]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:20:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:29] expected success, got: exit code: 101
[00:20:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1075:9
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0080f462:start=1522894067532624521,finish=1522894067540015082,duration=7390561
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:172d5d91
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
