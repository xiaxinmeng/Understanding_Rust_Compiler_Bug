plain
[01:34:54] 
[01:34:54] ---- [ui (nll)] ui/emit-directives.rs stdout ----
[01:34:54] diff of stderr:
[01:34:54] 
[01:34:54] - {"directive":"metadata file written: .../emit-directives/a"}
[01:34:54] + {"directive":"metadata file written: .../emit-directives.nll/a"}
[01:34:54] 
[01:34:54] 
[01:34:54] The actual stderr differed from the expected stderr.
[01:34:54] The actual stderr differed from the expected stderr.
[01:34:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/emit-directives.nll/emit-directives.nll.stderr
[01:34:54] To update references, rerun the tests and pass the `--bless` flag
[01:34:54] To only update this specific test, also pass `--test-args emit-directives.rs`
[01:34:54] error: 1 errors occurred comparing output.
[01:34:54] status: exit code: 0
[01:34:54] status: exit code: 0
[01:34:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/emit-directives.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/emit-directives.nll/a" "-Zborrowck=migrate" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--emit=metadata" "--error-format=json" "-Z" "emit-directives" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/emit-directives.nll/auxiliary" "-A" "unused"
[01:34:54] ------------------------------------------
[01:34:54] 
[01:34:54] ------------------------------------------
[01:34:54] stderr:
---
[01:34:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:34:54] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:54] 
[01:34:54] 
[01:34:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:34:54] 
[01:34:54] 
[01:34:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:54] Build completed unsuccessfully in 0:08:58
[01:34:54] Build completed unsuccessfully in 0:08:58
[01:34:54] make: *** [check] Error 1
[01:34:54] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:075a1b26
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 29 09:47:17 UTC 2019
---
travis_time:end:02a6da16:start=1556531239736401888,finish=1556531239743397335,duration=6995447
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:001c8519
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:121fe424
travis_time:start:121fe424
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a30b332
$ dmesg | grep -i kill
