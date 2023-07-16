plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:05:00] tidy error: /checkout/src/librustc/ty/maps/plumbing.rs:124: line longer than 100 chars
[00:05:02] some tidy checks failed
[00:05:02]
[00:05:02]
[00:05:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:02] expected success, got: exit code: 1
[00:05:02]
[00:05:02]
[00:05:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:02] Build completed unsuccessfully in 0:01:52
[00:05:02] make: *** [tidy] Error 1
[00:05:02] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0d064240:start=1523023776511891361,finish=1523023776521780964,duration=9889603
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:12156fc8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:12156fc8:start=1523023776530212822,finish=1523023776539053659,duration=8840837
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0af8f169
$ dmesg | grep -i kill
[   11.751274] init: failsafe main process (1092) killed by TERM signal
