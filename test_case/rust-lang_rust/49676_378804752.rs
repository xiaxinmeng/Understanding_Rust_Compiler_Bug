plain
[00:00:45] configure: rust.quiet-tests     := True
---
[00:26:46] error: unused variable: `arg_0`
[00:26:46]   --> libcore/ops/range.rs:48:38
[00:26:46]    |
[00:26:46] 48 | #[derive(Copy, Clone, PartialEq, Eq, Hash)]
[00:26:46]    |                                      ^^^^ help: consider using `_arg_0` instead
---
[00:26:46]    = note: #[deny(unused_variables)] implied by #[deny(warnings)]
[00:26:46]
[00:26:46] error: unused variable: `arg_0`
[00:26:46]     --> libcore/option.rs:1219:62
[00:26:46]      |
[00:26:46] 1219 | #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
[00:26:46]      |                                                              ^^^^ help: consider using `_arg_0` instead
[00:26:46]
[00:26:46] error: unused variable: `arg_0`
[00:26:46]    --> libcore/fmt/mod.rs:104:43
[00:26:46]     |
[00:26:46] 104 | #[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
[00:26:46]     |                                           ^^^^ help: consider using `_arg_0` instead
---
[00:26:46]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:26:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:26:46] expected success, got: exit code: 101
[00:26:46] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1075:9
[00:26:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:26:46] travis_fold:end:stage1-std
[00:26:46] travis_time:end:stage1-std:start=1522896251938877462,finish=1522896304702149128,duration=52763271666
[00:26:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:46] Build completed unsuccessfully in 0:21:03
[00:26:46] make: *** [all] Error 1
[00:26:46] Makefile:28: recipe for target 'all' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0b4036c2:start=1522896305293935512,finish=1522896305302693988,duration=8758476
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0ad5f5a6
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0ad5f5a6:start=1522896305310069248,finish=1522896305317522163,duration=7452915
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09350d47
$ dmesg | grep -i kill
[   10.683938] init: failsafe main process (1093) killed by TERM signal
