plain
[00:00:48] configure: rust.quiet-tests     := True
---
[00:03:11] error[E0277]: the trait bound `for<'r> str::IsAsciiWhitespace: ops::function::FnMut<(&'r u8,)>` is not satisfied
[00:03:11]     --> libcore/str/mod.rs:2635:5
[00:03:11]      |
[00:03:11] 2635 |     inner: Map<Filter<SliceSplit<'a, u8, IsAsciiWhitespace>, IsNotEmpty>, BytesToStr>,
[00:03:11]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'r> ops::function::FnMut<(&'r u8,)>` is not implemented for `str::IsAsciiWhitespace`
[00:03:11]      |
[00:03:11] note: required by `slice::Split`
[00:03:11]     --> libcore/slice/mod.rs:1655:1
[00:03:11]      |
[00:03:11] 1655 | pub struct Split<'a, T:'a, P> where P: FnMut(&T) -> bool {
---
[00:03:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:14] expected success, got: exit code: 101
[00:03:14] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:03:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:03:14] travis_fold:end:stage0-std
[00:03:14] travis_time:end:stage0-std:start=1523826888830573051,finish=1523826900542284021,duration=11711710970
[00:03:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:14] Build completed unsuccessfully in 0:00:12
[00:03:14] Makefile:79: recipe for target 'tidy' failed
[00:03:14] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:1e050dde:start=1523826901630436855,finish=1523826901637034378,duration=6597523
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:012eeec5
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:012eeec5:start=1523826901642919086,finish=1523826901648654846,duration=5735760
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a23cb7b
$ dmesg | grep -i kill
[   10.398374] init: failsafe main process (1092) killed by TERM signal
