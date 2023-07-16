plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:05:14] tidy error: /checkout/src/libstd/primitive_docs.rs:178: trailing whitespace
[00:05:15] some tidy checks failed
[00:05:15]
[00:05:15]
[00:05:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:15] expected success, got: exit code: 1
[00:05:15]
[00:05:15]
[00:05:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:15] Build completed unsuccessfully in 0:02:02
[00:05:15] make: *** [tidy] Error 1
[00:05:15] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:2bb44d5a:start=1523828998949416604,finish=1523828998956771530,duration=7354926
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:056baa30
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:056baa30:start=1523828998962745853,finish=1523828998969647555,duration=6901702
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:060dfafd
$ dmesg | grep -i kill
[   10.817069] init: failsafe main process (1092) killed by TERM signal
