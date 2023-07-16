plain
[00:44:17] running 1469 tests
[00:44:21] .........................................................................................i..........
[00:44:26] ...............................................i....................................................
[00:44:30] ....................................................................................................
[00:44:33] .............................................................F......................................
[00:44:36] ......F.............................................................................................
[00:44:44] ....................................................................................................
[00:44:48] ....................................................................................................
[00:44:52] ....................................................................................................
[00:44:58] .....................................................................................i..............
---
[00:45:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:21] 
[00:45:21] ---- [ui] ui/error-codes/E0494.rs stdout ----
[00:45:21] 
[00:45:21] error: ui test compiled successfully!
[00:45:21] status: exit code: 0
[00:45:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0494.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0494/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0494/auxiliary" "-A" "unused"
[00:45:21] ------------------------------------------
[00:45:21] 
[00:45:21] ------------------------------------------
[00:45:21] stderr:
---
[00:45:21] test result: FAILED. 1453 passed; 2 failed; 14 ignored; 0 measured; 0 filtered out
[00:45:21] 
[00:45:21] 
[00:45:21] 
[00:45:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:21] 
[00:45:21] 
[00:45:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:21] Build completed unsuccessfully in 0:02:30
[00:45:21] Build completed unsuccessfully in 0:02:30
[00./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
104168 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
103612 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
103236 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103232 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f1fkbhre4e-1xnyovl-17uifkj4ft9sj
92240 ./obj/build/x86_64-unknown-linux-gnu/stage1
92216 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90704 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
89808 ./src/llvm/test/CodeGen
