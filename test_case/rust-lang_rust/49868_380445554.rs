plain
[00:00:49] configure: rust.quiet-tests     := True
---
[00:04:58] tidy error: /checkout/src/libstd/path.rs:1496: mismatches to previous in: ["since"]
[00:04:58] tidy error: /checkout/src/libstd/path.rs:1503: mismatches to previous in: ["since"]
[00:04:58] tidy error: /checkout/src/libstd/path.rs:2490: mismatches to previous in: ["since"]
[00:04:58] tidy error: /checkout/src/libstd/path.rs:2497: mismatches to previous in: ["since"]
[00:04:59] some tidy checks failed
[00:04:59]
[00:04:59]
[00:04:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:59] expected success, got: exit code: 1
[00:04:59]
[00:04:59]
[00:04:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:59] Build completed unsuccessfully in 0:01:55
[00:04:59] Makefile:79: recipe for target 'tidy' failed
[00:04:59] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:104ce914:start=1523452151645583261,finish=1523452151653736476,duration=8153215
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0526cb85
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0526cb85:start=1523452151661886551,finish=1523452151670246274,duration=8359723
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b0abae4
$ dmesg | grep -i kill
[   10.409696] init: failsafe main process (1092) killed by TERM signal
