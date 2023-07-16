plain
[01:08:14] 
[01:08:14] ---- [ui] ui/emit-directives.rs stdout ----
[01:08:14] diff of stderr:
[01:08:14] 
[01:08:14] - {"directive":"metadata file written: $TEST_BUILD_DIR/emit-directives/a"}
[01:08:14] + {"directive":"metadata file written: $TEST_BUILD_DIR/emit-directives/a.wasm"}
[01:08:14] 
[01:08:14] 
[01:08:14] The actual stderr differed from the expected stderr.
[01:08:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/emit-directives/emit-directives.stderr
[01:08:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/emit-directives/emit-directives.stderr
[01:08:14] To update references, rerun the tests and pass the `--bless` flag
[01:08:14] To only update this specific test, also pass `--test-args emit-directives.rs`
[01:08:14] error: 1 errors occurred comparing output.
[01:08:14] status: exit code: 0
[01:08:14] status: exit code: 0
[01:08:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/emit-directives.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/emit-directives/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--emit=metadata" "--error-format=json" "-Z" "emit-directives" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/emit-directives/auxiliary" "-A" "unused"
[01:08:14] ------------------------------------------
[01:08:14] 
[01:08:14] ------------------------------------------
[01:08:14] stderr:
---
[01:08:14] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:08:14] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:14] 
[01:08:14] 
[01:08:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:14] 
[01:08:14] 
[01:08:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:08:14] Build completed unsuccessfully in 0:59:40
---
travis_time:end:0c1220a6:start=1556515055842156923,finish=1556515055851590431,duration=9433508
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b624c0a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:24902c43
travis_time:start:24902c43
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23d97ca4
$ dmesg | grep -i kill
