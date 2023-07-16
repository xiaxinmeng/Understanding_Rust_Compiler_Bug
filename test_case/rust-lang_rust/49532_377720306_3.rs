ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:04:50]
[00:04:50]
[00:04:51] some tidy checks failed
[00:04:51]
[00:04:51]
[00:04:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:51] expected success, got: exit code: 1
[00:04:51]
[00:04:51]
[00:04:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:51] Build completed unsuccessfully in 0:02:04
[00:04:51] make: *** [tidy] Error 1
[00:04:51] Makefile:79: recipe for target 'tidy' failed
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:22a2a68f:start=1522527429816823904,finish=1522527429823929748,duration=7105844
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b9a9fbf
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1b9a9fbf:start=1522527429830172763,finish=1522527429836782326,duration=6609563
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a816479
$ dmesg | grep -i kill
[   11.029853] init: failsafe main process (1092) killed by TERM signal
