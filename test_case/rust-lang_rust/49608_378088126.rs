plain
[00:01:03] configure: rust.quiet-tests     := True
---
[00:06:37] tidy error: /checkout/src/liballoc/heap.rs:153: line longer than 100 chars
[00:06:37] tidy error: /checkout/src/liballoc/heap.rs:170: line longer than 100 chars
[00:06:37] tidy error: /checkout/src/libcore/heap.rs:695: line longer than 100 chars
[00:06:38] tidy error: /checkout/src/liballoc_system/lib.rs:174: line longer than 100 chars
[00:06:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:39] Build completed unsuccessfully in 0:02:47
[00:06:39] Makefile:79: recipe for target 'tidy' failed
[00:06:39] make: *** [tidy] Error 1
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:18010320:start=1522714655637117729,finish=1522714655647136197,duration=10018468
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2669a276
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2669a276:start=1522714655655879169,finish=1522714655664367478,duration=8488309
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21033209
$ dmesg | grep -i kill
[   11.445656] init: failsafe main process (1094) killed by TERM signal
