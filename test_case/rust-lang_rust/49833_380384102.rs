plain
Resolving deltas: 100% (613671/613671), completed with 4881 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:23:16] thread 'main' panicked at 'assertion failed: last_min_end <= position', librustc_metadata/encoder.rs:268:17
[00:23:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:17] error: Could not compile `core`.
[00:23:17]
[00:23:17] Caused by:
[00:23:17]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:23:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:17] expected success, got: exit code: 101
[00:23:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:23:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:17] travis_fold:end:stage1-std
[00:23:17] travis_time:end:stage1-std:start=1523437960816315660,finish=1523438017192265823,duration=56375950163
[00:23:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:17] Build completed unsuccessfully in 0:18:29
[00:23:17] Makefile:28: recipe for target 'all' failed
[00:23:17] make: *** [all] Error 1
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2912f28a:start=1523438017746254813,finish=1523438017754102010,duration=7847197
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18c7d788
$ dmesg | grep -i kill
[   10.128139] init: failsafe main process (1094) killed by TERM signal
