compile_fail,E0ed_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider adding an explicit lifetime bound `K: 'c`...","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2093-infer-outlives/enum.rs","byte_start":852,"byte_end":853,"line_start":29,"line_end":29,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"enum Ying<'c, K> {","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":"K: 'c","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0309]: the parameter type `K` may not live long enough\n  --> /checkout/src/test/ui/rfc-2093-infer-outlives/enum.rs:30:9\n   |\nLL | enum Ying<'c, K> {\n   |               - help: consider adding an explicit lifetime bound `K: 'c`...\nLL |     One(&'c Yang<K>) //~ ERROR the parameter type `K` may not live long enough [E0309]\n   |         ^^^^^^^^^^^\n   |\nnote: ...so that the reference type `&'c Yang<K>` does not outlive the data it points at\n  --> /checkout/src/test/ui/rfc-2093-infer-outlives/enum.rs:30:9\n   |\nLL |     One(&'c Yang<K>) //~ ERROR the parameter type `K` may not live long enough [E0309]\n   |         ^^^^^^^^^^^\n\n"}
[00:41:40] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:41:40] {"message":"For more information about this error, try `rustc --explain E0309`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0309`.\n"}
[00:41:40] ------------------------------------------
[00:41:40] 
[00:41:40] thread '[ui] ui/rfc-2093-infer-outlives/enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:41:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:41:40] test result: FAILED. 1378 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:41:40] 
[00:41:40] 
[00:41:40] 
[00:41:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:41:40] 
[00:41:40] 
[00:41:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:41:40] Build completed unsuccessfully in 0:02:17
[00:41:40] Build completed unsuccessfully in 0:02:17
[00:41:40] make: *** [check] Error 1
[00:41:40] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e28ca12
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
