plain
[00:55:51] status: exit code: 2
[00:55:51] command: "make"
[00:55:51] stdout:
[00:55:51] ------------------------------------------
[00:55:51] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small  foo.rs -C lto -O --target wasm32-unknown-unknown --cfg a
[00:55:51] wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small/foo.wasm
[00:55:51] 467
[00:55:51] [ "`wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small/foo.wasm`" -lt "1024" ]
[00:55:51] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small  foo.rs -C lto -O --target wasm32-unknown-unknown --cfg b
[00:55:51] wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small/foo.wasm
[00:55:51] 4028
[00:55:51] [ "`wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small/foo.wasm`" -lt "5120" ]
[00:55:51] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small  foo.rs -C lto -O --target wasm32-unknown-unknown --cfg c
[00:55:51] wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small/foo.wasm
[00:55:51] 2888
[00:55:51] [ "`wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small/foo.wasm`" -lt "5120" ]
[00:55:51] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small  foo.rs -C lto -O --target wasm32-unknown-unknown --cfg d
[00:55:51] Makefile:5: recipe for target 'all' failed
[00:55:51] ------------------------------------------
[00:55:51] stderr:
[00:55:51] ------------------------------------------
[00:55:51] error[E0308]: mismatched types
[00:55:51] error[E0308]: mismatched types
[00:55:51]   --> foo.rs:36:5
[00:55:51]    |
[00:55:51] 36 |     A.try_with(|x| x.replace(Vec::new()).len()).unwrap_or(0)
[00:55:51]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found usize
[00:55:51]    = note: expected type `()`
[00:55:51]               found type `usize`
[00:55:51] help: try adding a semicolon
[00:55:51]    |
[00:55:51]    |
[00:55:51] 36 |     A.try_with(|x| x.replace(Vec::new()).len()).unwrap_or(0);
[00:55:51] help: try adding a return type
[00:55:51]    |
[00:55:51] 33 | pub fn foo() -> usize {
[00:55:51]    |              ^^^^^^^^
[00:55:51]    |              ^^^^^^^^
[00:55:51] 
[00:55:51] error: aborting due to previous error
[00:55:51] 
[00:55:51] For more information about this error, try `rustc --explain E0308`.
[00:55:51] make: *** [all] Error 1
[00:55:51] ------------------------------------------
[00:55:51] 
[00:55:51] thread '[run-make] run-make/wasm-panic-small' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:55:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:55:51] 
[00:55:51] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:55:51] 
[00:55:51] 
[00:55:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-make" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:51] 
[00:55:51] 
[00:55:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:55:51] Build completed unsuccessfully in 0:52:33
---
travis_time:end:091cd4a2:start=1541077809019711314,finish=1541077809025544832,duration=5833518
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05160e63
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f51fd7b
travis_time:start:2f51fd7b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02374ce5
$ dmesg | grep -i kill
