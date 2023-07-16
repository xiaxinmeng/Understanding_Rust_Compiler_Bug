plain
Resolving deltas: 100% (607640/607640), completed with 4796 local objects.
---
[00:01:00] configure: rust.quiet-tests     := True
---
[00:20:07] error: can't qualify macro invocation with `pub`
[00:20:07]    --> libcore/mem.rs:950:1
[00:20:07]     |
[00:20:07] 950 | pub union ManuallyDrop<T>{ value: T }
[00:20:07]     | ^^^
[00:20:07]     |
[00:20:07]     = help: try adjusting the macro to put `pub` inside the invocation
[00:20:07]
[00:20:07] error: expected one of `!` or `::`, found `ManuallyDrop`
[00:20:07]    --> libcore/mem.rs:950:11
[00:20:07]     |
[00:20:07] 950 | pub union ManuallyDrop<T>{ value: T }
[00:20:07]     |           ^^^^^^^^^^^^ expected one of `!` or `::` here
---
[00:20:07]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:20:07] warning: build failed, waiting for other jobs to finish...
[00:20:13] error: build failed
[00:20:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:13] expected success, got: exit code: 101
[00:20:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:20:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:20:13] travis_fold:end:stage1-std
[00:20:13] travis_time:end:stage1-std:start=1522952697539010157,finish=1522952704299775411,duration=6760765254
[00:20:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:13] Build completed unsuccessfully in 0:15:23
[00:20:13] make: *** [all] Error 1
[00:20:13] Makefile:28: recipe for target 'all' failed
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:24036f96:start=1522952704994629569,finish=1522952705001253890,duration=6624321
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:072af594
$ dmesg | grep -i kill
[   10.255160] init: failsafe main process (1092) killed by TERM signal
