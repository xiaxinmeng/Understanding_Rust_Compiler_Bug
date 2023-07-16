plain
Resolving deltas: 100% (615027/615027), completed with 4886 local objects.
---
[00:00:51] configure: rust.quiet-tests     := True
---
[00:05:49] tidy error: /checkout/src/bootstrap/test.rs:927: line longer than 100 chars
[00:05:51] some tidy checks failed
[00:05:51]
[00:05:51]
[00:05:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:51] expected success, got: exit code: 1
[00:05:51]
[00:05:51]
[00:05:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:51] Build completed unsuccessfully in 0:02:27
[00:05:51] Makefile:79: recipe for target 'tidy' failed
[00:05:51] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0bfa28ef:start=1523783239475498643,finish=1523783239482607040,duration=7108397
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0a59e890
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0a59e890:start=1523783239488911685,finish=1523783239496142286,duration=7230601
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0339a260
$ dmesg | grep -i kill
[   10.599429] init: failsafe main process (1094) killed by TERM signal
