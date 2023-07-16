plain
Resolving deltas: 100% (614190/614190), completed with 4873 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:04:44] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1644: line longer than 100 chars
[00:04:46] some tidy checks failed
[00:04:46]
[00:04:46]
[00:04:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:46] expected success, got: exit code: 1
[00:04:46]
[00:04:46]
[00:04:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:46] Build completed unsuccessfully in 0:02:01
[00:04:46] make: *** [tidy] Error 1
[00:04:46] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:10060fb9:start=1523628706776695028,finish=1523628706783779426,duration=7084398
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0129d1fa
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0129d1fa:start=1523628706789950102,finish=1523628706796191299,duration=6241197
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:179425fb
$ dmesg | grep -i kill
[   10.942418] init: failsafe main process (1093) killed by TERM signal
