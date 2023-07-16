plain
Resolving deltas: 100% (605067/605067), completed with 4761 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:642: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:651: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:660: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:669: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:678: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:687: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:696: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:705: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:706: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/liballoc/tests/str.rs:707: line longer than 100 chars
[00:04:46] some tidy checks failed
[00:04:46]
[00:04:46]
[00:04:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:46] expected success, got: exit code: 1
[00:04:46]
[00:04:46]
[00:04:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:46] Build completed unsuccessfully in 0:01:53
[00:04:46] Makefile:79: recipe for target 'tidy' failed
[00:04:46] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0258ba80:start=1523916453831324825,finish=1523916453837939675,duration=6614850
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:03250e15
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:03250e15:start=1523916453843528410,finish=1523916453849607489,duration=6079079
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f619d66
$ dmesg | grep -i kill
[   10.250665] init: failsafe main process (1092) killed by TERM signal
