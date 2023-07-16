compile_fail,E658\n#[repr(u128)] // error: use of unstable l":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:43:58] ------------------------------------------
[00:43:58] 
[00:43:58] thread '[ui] ui/feature-gate-wasm_import_module.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:43:58] 
---
[00:43:58] test result: FAILED. 1445 passed; 6 failed; 14 ignored; 0 measured; 0 filtered out
[00:43:58] 
[00:43:58] 
[00:43:58] 
[00:43:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:58] 
[00:43:58] 
[00:43:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:58] Build completed unsuccessfully in 0:02:31
[00:43:58] Build completed unsuccessfully in 0:02:31
[00:43:58] Makefile:58: recipe for target 'check' failed
[00:43:58] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2dc64980
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
