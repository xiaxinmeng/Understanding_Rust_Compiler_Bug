plain
[00:50:54] ....................................................................................................
[00:50:58] ....................................................................................................
[00:51:00] ............................i.......................................................................
[00:51:03] ....................................................................................................
[00:51:05] .............................................................................iiiiiiiii..............
[00:51:11] i...................................................................................................
[00:51:14] ....................................................................................................
[00:51:17] ...........................................................i........................................
[00:51:20] ....................................................................................................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:20] 
[01:03:20] running 258 tests
[01:03:47] ...........F...........i............................................................................
E0080]: evaluation of constant value failed
[01:04:20]   --> /checkout/src/test/rustdoc/const-evalutation-ice.rs:20:23
[01:04:20]    |
[01:04:20]    |
[01:04:20] 20 | pub type _S = [usize; 0 - (mem::size_of::<S>() != 4) as usize];
[01:04:20]    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to subtract with overflow
[01:04:20] 
[01:04:20] ------------------------------------------
[01:04:20] 
[01:04:20] thread '[rustdoc] rustdoc/const-evalutation-ice.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
---
[01:04:20] test result: FAILED. 255 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:04:20] 
[01:04:20] 
[01:04:20] 
[01:04:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:20] 
[01:04:20] 
[01:04:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:20] Build completed unsuccessfully in 0:17:29
[01:04:20] Build completed unsuccessfully in 0:17:29
[01:04:20] Makefile:58: recipe for target 'check' failed
[01:04:20] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1310d220
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
