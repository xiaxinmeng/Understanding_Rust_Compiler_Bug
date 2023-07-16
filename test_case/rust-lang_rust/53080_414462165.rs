plain
[01:16:12] failures:
[01:16:12] 
[01:16:12] ---- [codegen] codegen/issue-13018.rs stdout ----
[01:16:12] 
[01:16:12] error: verification with 'FileCheck' failed
[01:16:12] status: exit code: 1
[01:16:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-13018/issue-13018.ll" "/checkout/src/test/codegen/issue-13018.rs"
[01:16:12] ------------------------------------------
[01:16:12] 
[01:16:12] ------------------------------------------
[01:16:12] stderr:
[01:16:12] stderr:
[01:16:12] ------------------------------------------
[01:16:12] /checkout/src/test/codegen/issue-13018.rs:17:15: error: CHECK-NOT: excluded string found in input
[01:16:12] // CHECK-NOT: __rust_dealloc
[01:16:12]               ^
[01:16:12] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-13018/issue-13018.ll:1560:13: note: found here
[01:16:12]  call void @__rust_dealloc(i8* %ptr, i64 %4, i64 %5)
[01:16:12] 
[01:16:12] ------------------------------------------
[01:16:12] 
[01:16:12] thread '[codegen] codegen/issue-13018.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
---
[01:16:12] test result: FAILED. 92 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out
[01:16:12] 
[01:16:12] 
[01:16:12] 
[01:16:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:12] 
[01:16:12] 
[01:16:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:12] Build completed unsuccessfully in 0:11:59
[01:16:12] Build completed unsuccessfully in 0:11:59
[01:16:12] Makefile:58: recipe for target 'check' failed
[01:16:12] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02dc3884
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
149120 ./src/llvm-emscripten/test
148688 ./obj/build/bootstrap/debug/incremental
144744 ./obj/build/x86_64-unknown-linux-gnu/test/ui
134256 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f41eu73y3c-1v5zkxn-1iplbejydu42a
128740 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123188 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
123184 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
121648 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
---
travis_time:end:2afad980:start=1534798874179600624,finish=1534798874192737111,duration=13136487
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0be319ac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b394fa8
travis_time:start:0b394fa8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07ae2920
$ dmesg | grep -i kill
