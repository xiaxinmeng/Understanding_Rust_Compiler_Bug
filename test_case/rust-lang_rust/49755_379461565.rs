plain
Resolving deltas: 100% (612901/612901), completed with 4871 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:04:41] tidy error: /checkout/src/test/ui/in-band-lifetimes/ellided-lifetimes-macro-checks.rs: missing trailing newline
[00:04:42] some tidy checks failed
[00:04:42]
[00:04:42]
[00:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:42] expected success, got: exit code: 1
[00:04:42]
[00:04:42]
[00:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:42] Build completed unsuccessfully in 0:02:00
[00:04:42] Makefile:79: recipe for target 'tidy' failed
[00:04:42] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:01a65ed8:start=1523099207403251722,finish=1523099207409899545,duration=6647823
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:26093c46
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:26093c46:start=1523099207416118481,finish=1523099207421918945,duration=5800464
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00677a71
$ dmesg | grep -i kill
[   11.691344] init: failsafe main process (1093) killed by TERM signal
