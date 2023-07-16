plain
Resolving deltas: 100% (614836/614836), completed with 4886 local objects.
---
[00:00:47] configure: rust.quiet-tests     := True
---
[00:05:20] tidy error: /checkout/src/librustdoc/html/static/settings.js: missing trailing newline
[00:05:21] some tidy checks failed
[00:05:21]
[00:05:21]
[00:05:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:21] expected success, got: exit code: 1
[00:05:21]
[00:05:21]
[00:05:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:21] Build completed unsuccessfully in 0:02:02
[00:05:21] make: *** [tidy] Error 1
[00:05:21] Makefile:79: recipe for target 'tidy' failed
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:13ee506e:start=1523653444149841583,finish=1523653444156873560,duration=7031977
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03961c93
$ dmesg | grep -i kill
[   10.902289] init: failsafe main process (1092) killed by TERM signal
