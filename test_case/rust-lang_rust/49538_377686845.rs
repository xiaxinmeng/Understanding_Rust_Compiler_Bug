plain
[00:00:45] configure: rust.quiet-tests     := True
---
[00:04:16] tidy error: /checkout/src/liballoc/rc.rs:476: line longer than 100 chars
[00:04:17] some tidy checks failed
[00:04:17]
[00:04:17]
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:17] expected success, got: exit code: 1
[00:04:17]
[00:04:17]
[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:17] Build completed unsuccessfully in 0:01:42
[00:04:17] Makefile:79: recipe for target 'tidy' failed
[00:04:17] make: *** [tidy] Error 1
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:027a6cf0:start=1522496127494102132,finish=1522496127502072619,duration=7970487
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0eeb4bd2
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0eeb4bd2:start=1522496127508813222,finish=1522496127516161157,duration=7347935
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1fc1f958
$ dmesg | grep -i kill
[   11.254369] init: failsafe main process (1094) killed by TERM signal
[   42.676843] init: plymouth-upstart-bridge main process (510) killed by TERM signal
