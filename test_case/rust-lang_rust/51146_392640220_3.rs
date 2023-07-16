\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-51102.rs","byte_start":1174,"byte_end":1182,"line_start":43,"line_end":43,"column_start":17,"column_end":25,"is_primary":true,"text":[{"text":"                state: 0","highlight_start":17,"highlight_end":25}],"label":"variant `SimpleEnum::NoState` does not have this field","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0026]: variant `SimpleEnum::NoState` does not have a field named `state`\n  --> /checkout/src/test/ui/issue-51102.rs:43:17\n   |\nLL |                 state: 0\n   |                 ^^^^^^^^ variant `SimpleEnum::NoState` does not have this field\n\n"}
[00:44:14] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:44:14] {"message":"Some errors occurred: E0025, E0026.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0025, E0026.\n"}
[00:44:14] {"message":"For more information about an error, try `rustc --explain E0025`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0025`.\n"}
[00:44:14] ------------------------------------------
[00:44:14] 
[00:44:14] thread '[ui] ui/issue-51102.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:44:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:44:14] 
[00:44:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:44:14] 
[00:44:14] 
[00:44:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:14] 
[00:44:14] 
[00:44:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:14] Build completed unsuccessfully in 0:02:29
[00:44:14] Build completed unsuccessfully in 0:02:29
[00:44:14] make: *** [check] Error 1
[00:44:14] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d221112
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
