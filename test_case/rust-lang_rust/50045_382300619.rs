plain
Resolving deltas: 100% (616733/616733), completed with 4914 local objects.
---
[00:00:53] configure: rust.quiet-tests     := True
---
[00:04:32] tidy error: /checkout/src/libsyntax/parse/parser.rs:2321: line longer than 100 chars
[00:04:32] tidy error: /checkout/src/librustc/hir/lowering.rs:3595: line longer than 100 chars
[00:04:33] some tidy checks failed
[00:04:33]
[00:04:33]
[00:04:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:33] expected success, got: exit code: 1
[00:04:33]
[00:04:33]
[00:04:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:33] Build completed unsuccessfully in 0:01:43
[00:04:33] make: *** [tidy] Error 1
[00:04:33] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:11077518:start=1524038574001235624,finish=1524038574008228997,duration=6993373
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0d4fb8e0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0d4fb8e0:start=1524038574014962247,finish=1524038574022335894,duration=7373647
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:026bc1e8
$ dmesg | grep -i kill
[   10.402637] init: failsafe main process (1096) killed by TERM signal
