plain
Receiving objects: 100% (163/163), 79.07 KiB | 15.81 MiB/s, done.
---
Resolving deltas: 100% (125/125), completed with 52 local objects.
---
[00:00:47] configure: rust.quiet-tests     := True
---
[00:04:32] tidy error: /checkout/src/libcore/unicode/mod.rs:11: mismatches to previous in: ["tracking issue"]
[00:04:32] tidy error: /checkout/src/libstd_unicode/lib.rs:23: mismatches to previous in: ["tracking issue"]
[00:04:33] some tidy checks failed
[00:04:33]
[00:04:33]
[00:04:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:33] expected success, got: exit code: 1
[00:04:33]
[00:04:33]
[00:04:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:33] Build completed unsuccessfully in 0:01:55
[00:04:33] Makefile:79: recipe for target 'tidy' failed
[00:04:33] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:131c7c48:start=1522950037728131176,finish=1522950037735954343,duration=7823167
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1e0369a8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
