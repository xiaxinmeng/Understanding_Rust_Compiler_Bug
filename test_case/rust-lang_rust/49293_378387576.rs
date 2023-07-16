plain
Resolving deltas: 100% (607705/607705), completed with 4799 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:04:09] tidy error: /checkout/src/tools/compiletest/src/main.rs:626: line longer than 100 chars
[00:04:10] some tidy checks failed
[00:04:10]
[00:04:10]
[00:04:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:10] expected success, got: exit code: 1
[00:04:10]
[00:04:10]
[00:04:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:10] Build completed unsuccessfully in 0:01:43
[00:04:10] Makefile:79: recipe for target 'tidy' failed
[00:04:10] make: *** [tidy] Error 1
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:00c3a60e:start=1522787404361827543,finish=1522787404369620476,duration=7792933
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:257549e8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:257549e8:start=1522787404376848300,finish=1522787404383568298,duration=6719998
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04641c62
$ dmesg | grep -i kill
[   11.067144] init: failsafe main process (1095) killed by TERM signal
