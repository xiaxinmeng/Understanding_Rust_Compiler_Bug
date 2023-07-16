plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:05:43] tidy error: /checkout/src/bootstrap/native.rs:495: line longer than 100 chars
[00:05:43] tidy error: /checkout/src/bootstrap/dist.rs:623: line longer than 100 chars
[00:05:43] tidy error: /checkout/src/bootstrap/dist.rs:789: line longer than 100 chars
[00:05:43] tidy error: /checkout/src/bootstrap/compile.rs:361: line longer than 100 chars
[00:05:45] some tidy checks failed
[00:05:45]
[00:05:45]
[00:05:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:45] expected success, got: exit code: 1
[00:05:45]
[00:05:45]
[00:05:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:45] Build completed unsuccessfully in 0:02:28
[00:05:45] Makefile:79: recipe for target 'tidy' failed
[00:05:45] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0742b226:start=1523749012108052664,finish=1523749012114153619,duration=6100955
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0b91b1f8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0b91b1f8:start=1523749012119908767,finish=1523749012126093756,duration=6184989
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1103a09f
$ dmesg | grep -i kill
[   10.569401] init: failsafe main process (1091) killed by TERM signal
