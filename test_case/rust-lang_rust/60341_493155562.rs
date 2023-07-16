plain
[01:13:56] failures:
[01:13:56] 
[01:13:56] ---- [compile-fail] compile-fail/issue-43733-2.rs stdout ----
[01:13:56] 
[01:13:56] error: /checkout/src/test/compile-fail/issue-43733-2.rs:24: unexpected error: '24:1: 24:36: `std::cell::Cell<bool>` cannot be shared between threads safely [E0277]'
[01:13:56] 
[01:13:56] error: /checkout/src/test/compile-fail/issue-43733-2.rs:24: expected error not found: `std::cell::Cell<std::thread::local::fast::DtorState>` cannot be shared between threads
[01:13:56] error: 1 unexpected errors found, 1 expected errors not found
[01:13:56] status: exit code: 1
[01:13:56] status: exit code: 1
[01:13:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-43733-2.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43733-2" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43733-2/auxiliary" "-A" "unused"
[01:13:56]     Error {
[01:13:56]         line_num: 24,
[01:13:56]         kind: Some(
[01:13:56]             Error,
[01:13:56]             Error,
[01:13:56]         ),
[01:13:56]         msg: "24:1: 24:36: `std::cell::Cell<bool>` cannot be shared between threads safely [E0277]",
[01:13:56] ]
[01:13:56] 
[01:13:56] not found errors (from test file): [
[01:13:56]     Error {
[01:13:56]     Error {
[01:13:56]         line_num: 24,
[01:13:56]         kind: Some(
[01:13:56]             Error,
[01:13:56]         ),
[01:13:56]         msg: "`std::cell::Cell<std::thread::local::fast::DtorState>` cannot be shared between threads",
[01:13:56] ]
[01:13:56] 
[01:13:56] thread '[compile-fail] compile-fail/issue-43733-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1405:13
[01:13:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:13:56] 
[01:13:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:13:56] 
[01:13:56] 
[01:13:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "compile-fail" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:56] 
[01:13:56] 
[01:13:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:13:56] Build completed unsuccessfully in 1:05:09
---
travis_time:end:210a2cc0:start=1558026885625713844,finish=1558026885637748304,duration=12034460
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:015b4fe8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f6740bf
travis_time:start:2f6740bf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03d100d0
$ dmesg | grep -i kill
