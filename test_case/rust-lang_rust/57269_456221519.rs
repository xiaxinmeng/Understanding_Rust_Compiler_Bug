plain
[01:38:10] failures:
[01:38:10] 
[01:38:10] ---- [codegen] codegen/simd-intrinsic-generic-bitmask.rs stdout ----
[01:38:10] 
[01:38:10] error: verification with 'FileCheck' failed
[01:38:10] status: exit code: 1
[01:38:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll" "/checkout/src/test/codegen/simd-intrinsic-generic-bitmask.rs"
[01:38:10] ------------------------------------------
[01:38:10] 
[01:38:10] ------------------------------------------
[01:38:10] stderr:
[01:38:10] stderr:
[01:38:10] ------------------------------------------
[01:38:10] /checkout/src/test/codegen/simd-intrinsic-generic-bitmask.rs:52:12: error: CHECK: expected string not found in input
[01:38:10]  // CHECK: %2 = lshr <16 x i8> %0, <i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7>
[01:38:10]            ^
[01:38:10] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:41:26: note: scanning from here
[01:38:10] define i16 @bitmask_int16(<16 x i8>* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
[01:38:10]                          ^
[01:38:10] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:45:2: note: possible intended match here
[01:38:10]  %1 = lshr <16 x i8> %0, <i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7>
[01:38:10] 
[01:38:10] ------------------------------------------
[01:38:10] 
[01:38:10] thread '[codegen] codegen/simd-intrinsic-generic-bitmask.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:38:10] test result: FAILED. 108 passed; 1 failed; 14 ignored; 0 measured; 0 filtered out
[01:38:10] 
[01:38:10] 
[01:38:10] 
[01:38:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:38:10] 
[01:38:10] 
[01:38:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:38:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:10] Build completed unsuccessfully in 0:19:51
[01:38:10] Makefile:48: recipe for target 'check' failed
[01:38:10] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dca2350
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 21 23:08:02 UTC 2019
---
travis_time:end:0c0a6b8a:start=1548112084471790077,finish=1548112084496113427,duration=24323350
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1510246d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23515a9f
travis_time:start:23515a9f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:009b88a8
$ dmesg | grep -i kill
