plain
Resolving deltas: 100% (616187/616187), completed with 4911 local objects.
---
[00:00:54] configure: rust.quiet-tests     := True
---
[00:04:46] tidy error: /checkout/src/test/run-make/cross-lang-lto/main.rs:12: tab character
[00:04:46] tidy error: /checkout/src/test/run-make/cross-lang-lto/lib.rs:13: tab character
[00:04:47] some tidy checks failed
[00:04:47]
[00:04:47]
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] expected success, got: exit code: 1
[00:04:47]
[00:04:47]
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:01:43
[00:04:47] Makefile:79: recipe for target 'tidy' failed
[00:04:47] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:01e73a4c:start=1523889833992680643,finish=1523889834000060425,duration=7379782
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1c5bb4fe
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1c5bb4fe:start=1523889834006828949,finish=1523889834014477806,duration=7648857
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09e6253c
$ dmesg | grep -i kill
[   11.013549] init: failsafe main process (1094) killed by TERM signal
