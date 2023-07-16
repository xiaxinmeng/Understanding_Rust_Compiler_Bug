plain
Resolving deltas: 100% (612667/612667), completed with 4869 local objects.
---
[00:01:04] configure: rust.quiet-tests     := True
---
[00:04:35] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1578: trailing whitespace
[00:04:35] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1582: trailing whitespace
[00:04:35] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1644: trailing whitespace
[00:04:36] some tidy checks failed
[00:04:36]
[00:04:36]
[00:04:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:36] expected success, got: exit code: 1
[00:04:36]
[00:04:36]
[00:04:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:36] Build completed unsuccessfully in 0:01:54
[00:04:36] Makefile:79: recipe for target 'tidy' failed
[00:04:36] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:159c8622:start=1523087670093303512,finish=1523087670099866766,duration=6563254
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:28befa78
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:28befa78:start=1523087670105669571,finish=1523087670112459931,duration=6790360
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05e4a96c
$ dmesg | grep -i kill
[   10.173691] init: failsafe main process (1094) killed by TERM signal
