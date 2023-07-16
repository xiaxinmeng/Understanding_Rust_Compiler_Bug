plain
[01:43:21] ---- [run-pass] run-pass/paths-containing-nul.rs stdout ----
[01:43:21] 
[01:43:21] error: test run failed!
[01:43:21] status: exit code: 101
[01:43:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/paths-containing-nul/a"
[01:43:21] ------------------------------------------
[01:43:21] ------------------------------------------
[01:43:21] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/paths-containing-nul/a", waiting for result
[01:43:21] ------------------------------------------
[01:43:21] stderr:
[01:43:21] ------------------------------------------
[01:43:21] ------------------------------------------
[01:43:21] thread 'main' panicked at 'copy2 returned a strange NotFound on a path with NUL', /checkout/src/test/run-pass/paths-containing-nul.rs:12:23
[01:43:21] 
[01:43:21] ------------------------------------------
[01:43:21] 
[01:43:21] thread '[run-pass] run-pass/paths-containing-nul.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:43:21] 
[01:43:21] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:43:21] 
[01:43:21] 
[01:43:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "run-pass" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:43:21] 
[01:43:21] 
[01:43:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
[01:43:21] Build completed unsuccessfully in 1:40:12
---
travis_time:end:1a6a3c63:start=1552694271800101533,finish=1552694271811990349,duration=11888816
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d6c7ac8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00b79145
travis_time:start:00b79145
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2df1e748
$ dmesg | grep -i kill
