plain
Resolving deltas: 100% (607638/607638), completed with 4801 local objects.
---
[00:00:43] configure: rust.quiet-tests     := True
---
[00:18:10] error: can't qualify macro invocation with `pub`
[00:18:10]    --> libcore/mem.rs:950:1
[00:18:10]     |
[00:18:10] 950 | pub union ManuallyDrop<T>{ value: T }
[00:18:10]     | ^^^
[00:18:10]     |
[00:18:10]     = help: try adjusting the macro to put `pub` inside the invocation
[00:18:10]
[00:18:10] error: expected one of `!` or `::`, found `ManuallyDrop`
[00:18:10]    --> libcore/mem.rs:950:11
[00:18:10]     |
[00:18:10] 950 | pub union ManuallyDrop<T>{ value: T }
[00:18:10]     |           ^^^^^^^^^^^^ expected one of `!` or `::` here
---
[00:18:10]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:18:10] warning: build failed, waiting for other jobs to finish...
[00:18:15] error: build failed
[00:18:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:18:15] expected success, got: exit code: 101
[00:18:15] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:18:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:18:15] travis_fold:end:stage1-std
[00:18:15] travis_time:end:stage1-std:start=1522963695553540715,finish=1522963701150493280,duration=5596952565
[00:18:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:15] Build completed unsuccessfully in 0:14:07
[00:18:15] make: *** [all] Error 1
[00:18:15] Makefile:28: recipe for target 'all' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0cb1a874:start=1522963701690521162,finish=1522963701696383041,duration=5861879
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:00278c08
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSi
