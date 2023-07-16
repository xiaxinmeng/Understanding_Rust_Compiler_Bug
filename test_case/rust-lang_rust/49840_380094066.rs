plain
Resolving deltas: 100% (614042/614042), completed with 4868 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:21:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:21:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:43] expected success, got: exit code: 101
[00:21:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:21:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:21:43] travis_fold:end:stage1-std
[00:21:43] travis_time:end:stage1-std:start=1523366143948361927,finish=1523366161171037840,duration=17222675913
[00:21:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:43] Build completed unsuccessfully in 0:16:45
[00:21:43] make: *** [all] Error 1
[00:21:43] Makefile:28: recipe for target 'all' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:13dff2cc:start=1523366161637153135,finish=1523366161643385547,duration=6232412
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:30ff1b28
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:30ff1b28:start=1523366161648856525,finish=1523366161654690857,duration=5834332
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18a3e3c4
$ dmesg | grep -i kill
[   10.409578] init: failsafe main process (1092) killed by TERM signal
