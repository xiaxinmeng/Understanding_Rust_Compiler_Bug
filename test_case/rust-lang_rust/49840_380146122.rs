plain
Resolving deltas: 100% (613567/613567), completed with 4873 local objects.
---
[00:00:48] configure: rust.quiet-tests     := True
---
[00:22:25] thread 'main' panicked at 'attempt to subtract with overflow', librustc/ty/fold.rs:481:42
[00:22:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:22:26] error: Could not compile `core`.
[00:22:26]
[00:22:26] Caused by:
[00:22:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:22:26] warning: build failed, waiting for other jobs to finish...
[00:22:29] error: build failed
[00:22:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:22:29] expected success, got: exit code: 101
[00:22:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:22:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:22:29] travis_fold:end:stage1-std
[00:22:29] travis_time:end:stage1-std:start=1523374612033433137,finish=1523374628614435963,duration=16581002826
[00:22:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:29] Build completed unsuccessfully in 0:17:06
[00:22:29] Makefile:28: recipe for target 'all' failed
[00:22:29] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:2daa37f0:start=1523374629252340580,finish=1523374629261588778,duration=9248198
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1cc367af
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1cc367af:start=1523374629270437201,finish=1523374629280286749,duration=9849548
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c010401
$ dmesg | grep -i kill
[   11.744187] init: failsafe main process (1093) killed by TERM signal
