plain
[00:22:41] error: failed to prepare thin LTO module: Invalid function metadata: outgoing forward refs (Producer: 'LLVM6.0.0' Reader: 'LLVM 6.0.0')
[00:22:41]
[00:22:41] error: aborting due to previous error
[00:22:41]
[00:22:41] [RUSTC-TIMING] alloc test:false 6.295
[00:22:41] error: Could not compile `alloc`.
[00:22:41]
[00:22:41] Caused by:
[00:22:41]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=8df28f61746cde0e -C extra-filename=-8df28f61746cde0e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabi/release/deps --target arm-unknown-linux-gnueabi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabi/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabi/release/deps/libcompiler_builtins-4d09826810ea27d9.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabi/release/deps/libcore-4a5ef9821ce55a79.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabi/release/build/compiler_builtins-b72835d5cba3fb05/out` (exit code: 101)
[00:22:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "arm-unknown-linux-gnueabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:22:41] expected success, got: exit code: 101
[00:22:41] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0d6c3651:start=1523647226743209616,finish=1523647226752630170,duration=9420554
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:2694dedd
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2694dedd:start=1523647226757226902,finish=1523647226764388896,duration=7161994
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02524558
$ dmesg | grep -i kill
[   10.200418] init: failsafe main process (1094) killed by TERM signal
