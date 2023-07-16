plain
Resolving deltas: 100% (612921/612921), completed with 4877 local objects.
---
[00:00:47] configure: rust.quiet-tests     := True
---
[00:05:03] tidy error: /checkout/src/librustc/traits/auto_trait.rs:76: line longer than 100 chars
[00:05:04] some tidy checks failed
[00:05:04]
[00:05:04]
[00:05:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:04] expected success, got: exit code: 1
[00:05:04]
[00:05:04]
[00:05:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:04] Build completed unsuccessfully in 0:02:01
[00:05:04] make: *** [tidy] Error 1
[00:05:04] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:01a9d426:start=1523527631091517032,finish=1523527631097636959,duration=6119927
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:08a6e160
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:08a6e160:start=1523527631103495261,finish=1523527631110054064,duration=6558803
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fd884f9
$ dmesg | grep -i kill
[   10.907031] init: failsafe main process (1096) killed by TERM signal
