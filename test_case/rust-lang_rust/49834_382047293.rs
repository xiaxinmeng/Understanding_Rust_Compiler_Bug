plain
[00:01:07] configure: rust.quiet-tests     := True
---
[00:42:45] thread 'main' panicked at 'assertion failed: *old == value', librustc_data_structures/sync.rs:241:42
---
[00:43:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:43:33] expected success, got: exit code: 101
[00:43:33]
[00:43:33]
[00:43:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:43:33] Build completed unsuccessfully in 0:05:47
[00:43:33] make: *** [all] Error 1
[00:43:33] Makefile:28: recipe for target 'all' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:2705b080:start=1523980841568033483,finish=1523980841575288725,duration=7255242
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:294834ea
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:294834ea:start=1523980841581578671,finish=1523980841588508966,duration=6930295
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09c17180
$ dmesg | grep -i kill
[   10.544864] init: failsafe main process (1093) killed by TERM signal
