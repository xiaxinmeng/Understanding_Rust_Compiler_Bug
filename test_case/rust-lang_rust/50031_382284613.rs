plain
Resolving deltas: 100% (616885/616885), completed with 4918 local objects.
---
[00:00:47] configure: rust.quiet-tests     := True
---
[00:05:28] tidy error: /checkout/src/test/compile-fail/issue32829.rs:23: line longer than 100 chars
[00:05:28] tidy error: /checkout/src/test/compile-fail/issue32829.rs:47: line longer than 100 chars
[00:05:28] tidy error: /checkout/src/test/compile-fail/issue32829.rs:71: line longer than 100 chars
[00:05:29] some tidy checks failed
[00:05:29]
[00:05:29]
[00:05:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:29] expected success, got: exit code: 1
[00:05:29]
[00:05:29]
[00:05:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:29] Build completed unsuccessfully in 0:01:56
[00:05:29] Makefile:79: recipe for target 'tidy' failed
[00:05:29] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:10a9d8e2:start=1524035008251556297,finish=1524035008258891418,duration=7335121
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0c496e0a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0c496e0a:start=1524035008266569877,finish=1524035008273576047,duration=7006170
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10a03616
$ dmesg | grep -i kill
[   11.244002] init: failsafe main process (1096) killed by TERM signal
