plain
[00:50:13] ...............................................................................................i.... 2200/4550
[00:50:18] .................................................................................................... 2300/4550
[00:50:21] .................................................................................................... 2400/4550
[00:50:25] .................................................................................................... 2500/4550
[00:50:29] .......iiiiiiiii.................................................................................... 2600/4550
[00:50:35] .................................................................................................... 2800/4550
[00:50:39] .................................................................................................... 2900/4550
[00:50:42] ..........................i......................................................................... 3000/4550
[00:50:45] ......................................................................................i.i..ii....... 3100/4550
---
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:40] 
[01:02:40] running 273 tests
/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:43] 
[01:02:43] 
[01:02:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:43] Build completed unsuccessfully in 0:17:13
[01:02:43] Build completed unsuccessfully in 0:17:13
[01:02:43] Makefile:58: recipe for target 'check' failed
[01:02:43] make: *** [check] Error 1
2839804 ./obj
2626236 ./obj/build
1993724 ./obj/build/x86_64-unknown-linux-gnu
1069132 ./src
