plain
[01:20:26] 
[01:20:26] ---- [ui (nll)] ui/save-analysis/emit-notifications.rs stdout ----
[01:20:26] diff of stderr:
[01:20:26] 
[01:20:26] - {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/save-analysis/libemit_notifications.json","emit":"save-analysis"}
[01:20:26] - {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/libemit_notifications.rlib","emit":"link"}
[01:20:26] + {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications.nll/save-analysis/libemit_notifications.json","emit":"save-analysis"}
[01:20:26] + {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications.nll/libemit_notifications.rlib","emit":"link"}
[01:20:26] 
[01:20:26] 
[01:20:26] The actual stderr differed from the expected stderr.
[01:20:26] The actual stderr differed from the expected stderr.
[01:20:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications.nll/emit-notifications.nll.stderr
[01:20:26] To update references, rerun the tests and pass the `--bless` flag
[01:20:26] To only update this specific test, also pass `--test-args save-analysis/emit-notifications.rs`
[01:20:26] error: 1 errors occurred comparing output.
[01:20:26] status: exit code: 0
[01:20:26] status: exit code: 0
[01:20:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/save-analysis/emit-notifications.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-Zemit-artifact-notifications" "--crate-type" "rlib" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications.nll/auxiliary" "-A" "unused"
[01:20:26] ------------------------------------------
[01:20:26] 
[01:20:26] ------------------------------------------
[01:20:26] stderr:
---
[01:20:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:20:26] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:20:26] 
[01:20:26] 
[01:20:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.37.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:20:26] 
[01:20:26] 
[01:20:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:26] Build completed unsuccessfully in 1:17:13
---
travis_time:end:0ca06ce6:start=1559722716237748666,finish=1559722716244085446,duration=6336780
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:023b01fd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bca75a4
travis_time:start:1bca75a4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:399fa02a
$ dmesg | grep -i kill
