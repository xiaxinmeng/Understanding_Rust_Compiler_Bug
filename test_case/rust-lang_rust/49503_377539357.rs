plain
Resolving deltas: 100% (613124/613124), completed with 4853 local objects.
---
[00:00:45] configure: rust.quiet-tests     := True
---
[00:23:53] error[E0463]: can't find crate for `compiler_builtins`
[00:23:53]
[00:23:53] error: aborting due to previous error
[00:23:53]
[00:23:53] For more information about this error, try `rustc --explain E0463`.
[00:23:53] error: Could not compile `unwind`.
[00:23:53]
[00:23:53] Caused by:
[00:23:53]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name unwind libunwind/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=09bb8c86b54eb662 -C extra-filename=-09bb8c86b54eb662 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-4b35f2d05e20a0a4.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-a8294e31ba20c477.rlib -l gcc_s -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-3fdfe34106cc8b15/out` (exit code: 101)
[00:23"always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:54] expected success, got: exit code: 101
[00:23:54] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:23:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:54] travis_fold:end:stage1-std
[00:23:54] travis_time:end:stage1-std:start=1522421066798884900,finish=1522421136842379243,duration=70043494343
[00:23:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:54] Build completed unsuccessfully in 0:18:40
[00:23:54] Makefile:28: recipe for target 'all' failed
[00:23:54] make: *** [all] Error 1
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:17bed2ce:start=1522421137766754712,finish=1522421137778643452,duration=11888740
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0da2802c
$ dmesg | grep -i kill
[   11.516288] init: failsafe main process (1093) killed by TERM signal
