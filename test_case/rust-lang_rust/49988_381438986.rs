plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:05:40] tidy error: /checkout/src/libstd/primitive_docs.rs:178: trailing whitespace
[00:05:41] some tidy checks failed
[00:05:41]
[00:05:41]
[00:05:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:41] expected success, got: exit code: 1
[00:05:41]
[00:05:41]
[00:05:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:41] Build completed unsuccessfully in 0:02:32
[00:05:41] make: *** [tidy] Error 1
[00:05:41] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:15693899:start=1523827160198752133,finish=1523827160206541759,duration=7789626
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0240e069
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0240e069:start=1523827160213176373,finish=1523827160220525100,duration=7348727
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:35dadb14
$ dmesg | grep -i kill
[   10.523986] init: failsafe main process (1095) killed by TERM signal
