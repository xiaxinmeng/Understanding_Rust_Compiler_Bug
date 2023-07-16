plain
[00:23:36] error: failed to prepare thin LTO module: Invalid function metadata: outgoing forward refs (Producer: 'LLVM6.0.0' Reader: 'LLVM 6.0.0')
[00:23:36]
[00:23:36] error: aborting due to previous error
[00:23:36]
[00:23:36] [RUSTC-TIMING] alloc test:false 6.701
[00:23:36] error: Could not compile `alloc`.
[00:23:36]
[00:23:36] Caused by:
[00:23:36]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=f44c2b79c5c900d2 -C extra-filename=-f44c2b79c5c900d2 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/aarch64-unknown-linux-gnu/release/deps --target aarch64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/aarch64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/aarch64-unknown-linux-gnu/release/deps/libcompiler_builtins-0131a1f921771d4b.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/aarch64-unknown-linux-gnu/release/deps/libcore-e89ee5bd779a4348.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/aarch64-unknown-linux-gnu/release/build/compiler_builtins-dd154bc0b1d5dfeb/out` (exit code: 101)
[00:23:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:36] expected success, got: exit code: 101
[00:23:36] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:242aa404:start=1523816141901628648,finish=1523816141909396600,duration=7767952
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:06b16048
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:06b16048:start=1523816141916492114,finish=1523816141924556659,duration=8064545
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0374002e
$ dmesg | grep -i kill
[   10.476510] init: failsafe main process (1094) killed by TERM signal
