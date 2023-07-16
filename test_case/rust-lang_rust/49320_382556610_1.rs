\n\nPaths in `use` statements are relative to the crate root. To import items\nrelative to the current and parent modules, use the `self::` and `super::`\nprefixes, respectively. Also verify that you didn't misspell the import\nname a[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:19] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] thread '[ui] ui/allocator-submodule.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2930:9
[01:00:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:00:19] test result: FAILED. 1355 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[01:00:19] 
[01:00:19] 
[01:00:19] 
[01:00:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:19] 
[01:00:19] 
[01:00:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:19] Build completed unsuccessfully in 0:03:04
[01:00:19] Build completed unsuccessfully in 0:03:04
[01:00:19] make: *** [check] Error 1
[01:00:19] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05b61436
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
