plain
Receiving objects: 100% (161/161), 25.38 KiB | 25.38 MiB/s, done.
---
Resolving deltas: 100% (143/143), completed with 69 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:04:37] tidy error: /checkout/src/librustc_trans/debuginfo/metadata.rs:1570: TODO is deprecated; use FIXME
[00:04:37] tidy error: /checkout/src/librustc_mir/monomorphize/deduplicate_instances.rs:69: line longer than 100 chars
[00:04:37] tidy error: /checkout/src/librustc_mir/monomorphize/deduplicate_instances.rs:74: line longer than 100 chars
[00:04:37] tidy error: /checkout/src/librustc_mir/monomorphize/deduplicate_instances.rs:76: line longer than 100 chars
[00:04:37] tidy error: /checkout/src/librustc/ty/mod.rs: missing trailing newline
[00:04:38] some tidy checks failed
[00:04:38]
[00:04:38]
[00:04:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:38] expected success, got: exit code: 1
[00:04:38]
[00:04:38]
[00:04:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:38] Build completed unsuccessfully in 0:01:46
[00:04:38] make: *** [tidy] Error 1
[00:04:38] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:066a3e80:start=1523120824857936064,finish=1523120824865114436,duration=7178372
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04689fa7
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:04689fa7:start=1523120824871548739,finish=1523120824878148085,duration=6599346
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01b48d56
$ dmesg | grep -i kill
[   10.800923] init: failsafe main process (1097) killed by TERM signal
