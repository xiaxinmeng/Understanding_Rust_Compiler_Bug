plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:06:14] tidy error: /checkout/src/libsyntax_pos/symbol.rs:399: line longer than 100 chars
[00:06:15] some tidy checks failed
[00:06:15]
[00:06:15]
[00:06:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:15] expected success, got: exit code: 1
[00:06:15]
[00:06:15]
[00:06:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:15] Build completed unsuccessfully in 0:02:36
[00:06:15] Makefile:79: recipe for target 'tidy' failed
[00:06:15] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:05f4146a:start=1523499577381273781,finish=1523499577389599607,duration=8325826
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:09b82c50
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:09b82c50:start=1523499577397309795,finish=1523499577406013046,duration=8703251
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3324f878
$ dmesg | grep -i kill
[   11.571928] init: failsafe main process (1094) killed by TERM signal
