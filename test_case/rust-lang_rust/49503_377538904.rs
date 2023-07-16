plain
Resolving deltas: 100% (613128/613128), completed with 4853 local objects.
---
[00:00:43] configure: rust.quiet-tests     := True
---
[00:20:08] error[E0463]: can't find crate for `compiler_builtins`
[00:20:08]
[00:20:08] error: aborting due to previous error
[00:20:08]
[00:20:08] For more information about this error, try `rustc --explain E0463`.
[00:20:08] error: Could not compile `panic_abort`.
[00:20:08]
[00:20:08] Caused by:
[00:20:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name panic_abort libpanic_abort/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=14193139c52711af -C extra-filename=-14193139c52711af --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-4b35f2d05e20a0a4.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-a8294e31ba20c477.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-3fdfe34106cc8b15/out` (exit code: 101)
[00:20:08] warning: build failed, waiting for other jobs to finish...
[00:20:09] error: build failed
[00:20:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:09] expected success, got: exit code: 101
[00:20:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:20:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:20:09] travis_fold:end:stage1-std
[00:20:09] travis_time:end:stage1-std:start=1522420949020891945,finish=1522421002426650809,duration=53405758864
[00:20:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:09] Build completed unsuccessfully in 0:15:04
[00:20:09] Makefile:28: recipe for target 'all' failed
[00:20:09] make: *** [all] Error 1
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:048fa278:start=1522421003045336271,finish=1522421003052734791,duration=7398520
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:148da1bc
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:148da1bc:start=1522421003058907052,finish=1522421003065611621,duration=6704569
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0052db94
$ dmesg | grep -i kill
[   10.710744] init: failsafe main process (1092) killed by TERM signal
