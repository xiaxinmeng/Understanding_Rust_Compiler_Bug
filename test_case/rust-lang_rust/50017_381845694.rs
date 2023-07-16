plain
[00:00:50] configure: rust.quiet-tests     := True
---
[00:03:06] error[E0541]: unknown meta item 'issue'
[00:03:06]    --> libcore/time.rs:168:43
[00:03:06]     |
[00:03:06] 168 |     #[stable(feature = "duration_extras", issue = "1.27.0")]
---
[00:03:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:27] expected success, got: exit code: 101
[00:03:27] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:03:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:03:27] travis_fold:end:stage0-std
[00:03:27] travis_time:end:stage0-std:start=1523942275425743987,finish=1523942301523247101,duration=26097503114
[00:03:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:27] Build completed unsuccessfully in 0:00:27
[00:03:27] Makefile:79: recipe for target 'tidy' failed
[00:03:27] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:081e1704:start=1523942302010125166,finish=1523942302017219719,duration=7094553
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04acdaa4
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:04acdaa4:start=1523942302023102973,finish=1523942302030158280,duration=7055307
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:174a5ce7
$ dmesg | grep -i kill
[   10.558849] init: failsafe main process (1094) killed by TERM signal
