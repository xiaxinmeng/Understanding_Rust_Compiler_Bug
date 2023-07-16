plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:03:09] error[E0277]: the trait bound `for<'r> str::IsAsciiWhitespace: ops::function::FnMut<(&'r u8,)>` is not satisfied
[00:03:09]     --> libcore/str/mod.rs:2635:5
[00:03:09]      |
[00:03:09] 2635 |     inner: Map<Filter<SliceSplit<'a, u8, IsAsciiWhitespace>, IsNotEmpty>, BytesToStr>,
[00:03:09]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'r> ops::function::FnMut<(&'r u8,)>` is not implemented for `str::IsAsciiWhitespace`
[00:03:09]      |
[00:03:09] note: required by `slice::Split`
[00:03:09]     --> libcore/slice/mod.rs:1655:1
[00:03:09]      |
[00:03:09] 1655 | pub struct Split<'a, T:'a, P> where P: FnMut(&T) -> bool {
---
[00:03:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:12] expected success, got: exit code: 101
[00:03:12] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:03:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:03:12] travis_fold:end:stage0-std
[00:03:12] travis_time:end:stage0-std:start=1523825587941191947,finish=1523825599238220358,duration=11297028411
[00:03:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:12] Build completed unsuccessfully in 0:00:12
[00:03:12] Makefile:79: recipe for target 'tidy' failed
[00:03:12] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0e665b96:start=1523825599726098956,finish=1523825599732533241,duration=6434285
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:15481c42
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:15481c42:start=1523825599738037634,finish=1523825599743859096,duration=5821462
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05d172d8
$ dmesg | grep -i kill
[   10.219692] init: failsafe main process (1096) killed by TERM signal
