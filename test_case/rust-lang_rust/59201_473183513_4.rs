\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-type.rs","byte_start":137,"byte_end":161,"line_start":8,"line_end":8,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"struct i64f64(i64, f64); //~ ERROR SIMD vector should be homogeneous","highlight_start":1,"highlight_end":25}],"label":"SIMD elements must have the same type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0076]: SIMD vector should be homogeneous\n  --> /checkout/src/test/ui/simd-type.rs:8:1\n   |\nLL | struct i64f64(i64, f64); //~ ERROR SIMD vector should be homogeneous\n   | ^^^^^^^^^^^^^^^^^^^^^^^^ SIMD elements must have the same type\n\n"}
[01:14:55] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:14:55] {"message":"Some errors occurred: E0075, E0076.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0075, E0076.\n"}
[01:14:55] 
[01:14:55] ------------------------------------------
[01:14:55] 
[01:14:55] thread '[ui] ui/simd-type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:14:55] 
[01:14:55] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:14:55] 
[01:14:55] 
[01:14:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:55] 
[01:14:55] 
[01:14:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:55] Build completed unsuccessfully in 0:04:28
[01:14:55] Build completed unsuccessfully in 0:04:28
[01:14:55] make: *** [check] Error 1
[01:14:55] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ded0401
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 15 07:18:44 UTC 2019
