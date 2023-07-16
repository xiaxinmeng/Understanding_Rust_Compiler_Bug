plain
Resolving deltas: 100% (616678/616678), completed with 4917 local objects.
---
[00:00:55] configure: rust.quiet-tests     := True
---
[00:05:13] tidy error: /checkout/src/libsyntax/attr.rs:1125: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/libsyntax/attr.rs:1128: line longer than 100 chars
[00:05:15] some tidy checks failed
[00:05:15]
[00:05:15]
[00:05:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:15] expected success, got: exit code: 1
[00:05:15]
[00:05:15]
[00:05:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:15] Build completed unsuccessfully in 0:01:45
[00:05:15] make: *** [tidy] Error 1
[00:05:15] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:16ad2bee:start=1523978430524380980,finish=1523978430531534202,duration=7153222
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:2435d030
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2435d030:start=1523978430537857351,finish=1523978430544687599,duration=6830248
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2a0706b6
$ dmesg | grep -i kill
[   10.757959] init: failsafe main process (1093) killed by TERM signal
[   42.095280] init: plymouth-upstart-bridge main process (510) killed by TERM signal
