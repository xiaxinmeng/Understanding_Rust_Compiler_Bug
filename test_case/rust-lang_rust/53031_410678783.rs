plain
[00:43:07] ....................................................................................................
[00:43:09] ....................................................................................................
[00:43:12] ....................................................................................................
[00:43:15] ....................................................................................................
[00:43:17] iiiiiiiii...........................................................................................
[00:43:23] ....................................................................................................
[00:43:26] .....i..............................................................................................
[00:43:29] ..............i.....................................................................................
[00:43:31] ....................................................................................................
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:34] 
[00:50:34] running 104 tests
heckout/obj/build/x86_64-unknown-linux-gnu/test/codegen/target-cpu-on-functions/target-cpu-on-functions.ll" "/checkout/src/test/codegen/target-cpu-on-functions.rs"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] /checkout/src/test/codegen/target-cpu-on-functions.rs:28:11: error: expected string not found in input
[00:50:37] // CHECK: attributes #0 = {{.*}} "target-cpu"="x86-64"
[00:50:37]           ^
[00:50:37] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/target-cpu-on-functions/target-cpu-on-functions.ll:19:104: note: scanning from here
[00:50:37] define internal void @_ZN23target_cpu_on_functions12not_exported17h20a13a30b2442844E() unnamed_addr #0 {
[00:50:37]                                                                                                        ^
[00:50:37] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/target-cpu-on-functions/target-cpu-on-functions.ll:24:1: note: possible intended match here
[00:50:37] attributes #0 = { nounwind "probe-stack"="__rust_probestack" }
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[codegen] codegen/target-cpu-on-functions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
---
[00:50:37] test result: FAILED. 75 passed; 1 failed; 28 ignored; 0 measured; 0 filtered out
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:37] 
[00:50:37] 
[00:50:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:37] Build completed unsuccessfully in 0:10:01
[00:50:37] Build completed unsuccessfully in 0:10:01
[00:50:37] make: *** [check] Error 1
[00:50:37] Makefile:58: recipe for target 'check' failed
34524 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build
34496 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build
34344 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
34336 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
