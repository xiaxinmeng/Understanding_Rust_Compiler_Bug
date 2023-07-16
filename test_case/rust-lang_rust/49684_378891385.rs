plain
[00:26:50] error[E0463]: can't find crate for `compiler_builtins`
[00:26:50]
[00:26:50] error: aborting due to previous error
[00:26:50]
[00:26:50] For more information about this error, try `rustc --explain E0463`.
[00:26:50] [RUSTC-TIMING] profiler_builtins test:false 0.121
[00:26:50] The following warnings were emitted during compilation:
---
[00:26:50] error: Could not compile `profiler_builtins`.
[00:26:50]
[00:26:50] Caused by:
[00:26:50]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name profiler_builtins libprofiler_builtins/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=b56639074230e1d1 -C extra-filename=-b56639074230e1d1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-6fe704c3bfbe80d2.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-ac848cf491139595/out -l static=profiler-rt` (exit code: 101)
[00:26:50] warning: build failed, waiting for other jobs to finish...
[00:26:53] [RUSTC-TIMING] compiler_builtins test:false 2.936
[00:28:07] error: build failed
[00:28:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:28:07] expected success, got: exit code: 101
[00:28:07] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:11aa78f8:start=1522924131545412675,finish=1522924131553056385,duration=7643710
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:02c3477a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:02c3477a:start=1522924131558347480,finish=1522924131564187228,duration=5839748
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:072af3ff
$ dmesg | grep -i kill
[   10.580310] init: failsafe main process (1093) killed by TERM signal
