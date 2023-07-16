\n\nIf you're using a stable or a beta version of rustc, you won't be able to use\nany unstable features. In order to do so, please switch to a nightly version of\nr tools/compiletest/src/runtest.rs:3284:9
[00:52:14] 
[00:52:14] 
[00:52:14] failures:
[00:52:14]     [ui] ui/consts/const-eval/mod-static-with-const-fn.rs
[00:52:14]     [ui] ui/consts/const-eval/mod-static-with-const-fn.rs
[00:52:14] 
[00:52:14] test result: FAILED. 4908 passed; 1 failed; 22 ignored; 0 measured; 0 filtered out
[00:52:14] 
[00:52:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:52:14] 
[00:52:14] 
[00:52:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:14] 
[00:52:14] 
[00:52:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:14] Build completed unsuccessfully in 0:03:42
[00:52:14] Build completed unsuccessfully in 0:03:42
[00:52:14] Makefile:58: recipe for target 'check' failed
[00:52:14] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1871ebe3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
