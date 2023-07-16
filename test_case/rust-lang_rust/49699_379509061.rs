plain
[00:23:29] error: can't qualify macro invocation with `pub`
[00:23:29]    --> libcore/mem.rs:950:1
[00:23:29]     |
[00:23:29] 950 | pub union ManuallyDrop<T>{ value: T }
[00:23:29]     | ^^^
[00:23:29]     |
[00:23:29]     = help: try adjusting the macro to put `pub` inside the invocation
[00:23:29]
[00:23:29] error: expected one of `!` or `::`, found `ManuallyDrop`
[00:23:29]    --> libcore/mem.rs:950:11
[00:23:29]     |
[00:23:29] 950 | pub union ManuallyDrop<T>{ value: T }
[00:23:29]     |           ^^^^^^^^^^^^ expected one of `!` or `::` here
---
[00:23:29]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:23:29] warning: build failed, waiting for other jobs to finish...
[00:23:36] error: build failed
[00:23:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:36] expected success, got: exit code: 101
[00:23:36] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:23:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:36] travis_fold:end:stage1-std
[00:23:36] travis_time:end:stage1-std:start=1523147249191398064,finish=1523147256197301359,duration=7005903295
[00:23:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:36] Build completed unsuccessfully in 0:18:19
[00:23:36] Makefile:28: recipe for target 'all' failed
[00:23:36] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:00a62472:start=1523147256869234073,finish=1523147256876769971,duration=7535898
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:24bb66da
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:24bb66da:start=1523147256888724724,finish=1523147256896296045,duration=7571321
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a45dac0
$ dmesg | grep -i kill
[   11.010625] init: failsafe main process (1095) killed by TERM signal
