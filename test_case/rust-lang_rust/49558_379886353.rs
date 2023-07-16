plain
Receiving objects: 100% (242/242), 24.45 KiB | 24.45 MiB/s, done.
---
Resolving deltas: 100% (214/214), completed with 66 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:40:56] thread 'main' panicked at 'assertion failed: *old == value', librustc_data_structures/sync.rs:241:42
---
[00:41:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:41:36] expected success, got: exit code: 101
[00:41:36]
[00:41:36]
[00:41:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:41:36] Build completed unsuccessfully in 0:05:37
[00:41:36] Makefile:28: recipe for target 'all' failed
[00:41:36] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:05cb855c:start=1523306285288881537,finish=1523306285310894001,duration=22012464
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:29038c1d
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
