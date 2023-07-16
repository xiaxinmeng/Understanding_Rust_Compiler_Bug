plain
[00:00:48] configure: rust.quiet-tests     := True
---
e-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib` (exit code: 101)
[00:05:20] command did not execute successfully: "/checkout/obj/build/base/x86_64-unknown-linux-gnu/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:20] expected success, got: exit code: 101
[00:05:20] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1156:9
[00:05:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:05:20] travis_fold:end:stage0-rustc
[00:05:20] travis_time:end:stage0-rustc:start=1523926792299933835,finish=1523926830456313927,duration=38156380092
[00:05:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:20] Build completed unsuccessfully in 0:01:24
[00:05:20] make: *** [all] Error 1
[00:05:20] Makefile:28: recipe for target 'all' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:14660380:start=1523926830909209788,finish=1523926830914539224,duration=5329436
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:076abb30
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:076abb30:start=1523926830919470856,finish=1523926830924782229,duration=5311373
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16fdbbfc
$ dmesg | grep -i kill
[   10.359798] init: failsafe main process (1093) killed by TERM signal
