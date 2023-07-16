plain
Receiving objects: 100% (304/304), 48.86 KiB | 24.43 MiB/s, done.
---
Resolving deltas: 100% (247/247), completed with 74 local objects.
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm.tar.gz &&         curl -sSL -o download-src-llvm.tar.gz https://github.com/rust-lang/llvm/archive/7243155b1c3da0a980c868a87adebf00e0b33989.tar.gz
---
[00:00:48] configure: rust.quiet-tests     := True
---
[00:04:11] * 179 features
[00:04:11] tidy error: /checkout/src/liballoc/oom.rs:19: platform-specific cfg: cfg(any(unix, target_os = "redox"))
[00:04:11] tidy error: /checkout/src/liballoc/oom.rs:67: platform-specific cfg: cfg(not(any(windows, unix, target_os = "redox")))
[00:04:12] some tidy checks failed
[00:04:12]
[00:04:12]
[00:04:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:12] expected success, got: exit code: 1
[00:04:12]
[00:04:12]
[00:04:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:12] Build completed unsuccessfully in 0:01:44
[00:04:12] Makefile:79: recipe for target 'tidy' failed
[00:04:12] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:00bac534:start=1522878325536986682,finish=1522878325544062106,duration=7075424
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:28f99d88
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:28f99d88:start=1522878325550815862,finish=1522878325559702359,duration=8886497
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d0ad1ac
$ dmesg | grep -i kill
[   10.868621] init: failsafe main process (1093) killed by TERM signal
