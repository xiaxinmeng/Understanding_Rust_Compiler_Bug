plain
[01:11:35] 
[01:11:35] ---- [ui] ui/save-analysis/emit-notifications.rs stdout ----
[01:11:35] diff of stderr:
[01:11:35] 
[01:11:35] 1 {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/save-analysis/emit_notifications.json","emit":"save-analysis"}
[01:11:35] - {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/emit-notifications","emit":"link"}
[01:11:35] + {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/emit-notifications.wasm","emit":"link"}
[01:11:35] 
[01:11:35] 
[01:11:35] The actual stderr differed from the expected stderr.
[01:11:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications/emit-notifications.stderr
[01:11:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications/emit-notifications.stderr
[01:11:35] To update references, rerun the tests and pass the `--bless` flag
[01:11:35] To only update this specific test, also pass `--test-args save-analysis/emit-notifications.rs`
[01:11:35] error: 1 errors occurred comparing output.
[01:11:35] status: exit code: 0
[01:11:35] status: exit code: 0
[01:11:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/save-analysis/emit-notifications.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-Zsave-analysis" "-Zemit-artifact-notifications" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications/auxiliary" "-A" "unused"
[01:11:35] ------------------------------------------
[01:11:35] 
[01:11:35] ------------------------------------------
[01:11:35] stderr:
---
[01:11:35] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:11:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:35] 
[01:11:35] 
[01:11:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.37.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:35] 
[01:11:35] 
[01:11:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:11:35] Build completed unsuccessfully in 1:02:52
---
travis_time:end:01e37981:start=1559589102827201834,finish=1559589102838585291,duration=11383457
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:057de91b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f9f0d50
travis_time:start:0f9f0d50
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01a2d7ee
$ dmesg | grep -i kill
