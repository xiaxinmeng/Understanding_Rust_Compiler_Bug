plain
[01:07:13] 
[01:07:13] running 258 tests
[01:07:40] .......................i............................................................................
[01:08:04] .........................i..........................................................................
 type Golf<T> where T: Clone = (T, T);
[01:08:13]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `T`
[01:08:13]    |
[01:08:13]    = help: consider adding a `where T: std::clone::Clone` bound
[01:08:13] error: Compilation failed, aborting rustdoc
[01:08:13] 
[01:08:13] 
[01:08:13] ------------------------------------------
---
[01:08:13] 
[01:08:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:08:13] 
[01:08:13] 
[01:08:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:13] 
[01:08:13] 
[01:08:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:13] Build completed unsuccessfully in 0:22:08
[01:08:13] Build completed unsuccessfully in 0:22:08
[01:08:13] Makefile:58: recipe for target 'check' failed
[01:08:13] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18c9276c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
