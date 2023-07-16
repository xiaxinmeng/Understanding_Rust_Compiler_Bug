plain
Resolving deltas: 100% (612604/612604), completed with 4871 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:04:26] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1595: trailing whitespace
[00:04:26] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1611: line longer than 100 chars
[00:04:28] some tidy checks failed
[00:04:28]
[00:04:28]
[00:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:28] expected success, got: exit code: 1
[00:04:28]
[00:04:28]
[00:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:28] Build completed unsuccessfully in 0:01:54
[00:04:28] make: *** [tidy] Error 1
[00:04:28] Makefile:79: recipe for target 'tidy' failed
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0137809c:start=1523025671971517003,finish=1523025671979159888,duration=7642885
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07f0b300
$ dmesg | grep -i kill
[   11.328129] init: failsafe main process (1095) killed by TERM signal
