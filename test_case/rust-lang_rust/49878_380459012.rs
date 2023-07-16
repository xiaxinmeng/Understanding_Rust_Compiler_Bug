plain
Resolving deltas: 100% (613823/613823), completed with 4888 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:04:36] tidy error: /checkout/src/librustc_trans/type_.rs:330: TODO is deprecated; use FIXME
[00:04:37] some tidy checks failed
[00:04:37]
[00:04:37]
[00:04:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:37] expected success, got: exit code: 1
[00:04:37]
[00:04:37]
[00:04:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:37] Build completed unsuccessfully in 0:01:45
[00:04:37] Makefile:79: recipe for target 'tidy' failed
[00:04:37] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0e2c5175:start=1523454640590169950,finish=1523454640597313231,duration=7143281
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:095160e0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:095160e0:start=1523454640603734434,finish=1523454640611305195,duration=7570761
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08c0cb7c
$ dmesg | grep -i kill
[   10.307371] init: failsafe main process (1093) killed by TERM signal
