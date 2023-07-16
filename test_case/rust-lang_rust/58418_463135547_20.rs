\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs","byte_start":1006,"byte_end":1007,"line_start":41,"line_end":41,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider adding an explicit lifetime bound `T: ReEarlyBound(0, 'a)`...","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0309]: the parameter type `T` may not live long enough\n  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:41:5\n   |\nLL |     x\n   |     ^\n   |\n   = help: consider adding an explicit lifetime bound `T: ReEarlyBound(0, 'a)`...\n\n"}
[01:01:00] {"message":"For more information about this error, try `rustc --explain E0309`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0309`.\n"}
[01:01:00] 
[01:01:00] ------------------------------------------
[01:01:00] 
---
[01:01:00] 
[01:01:00] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:01:00] 
[01:01:00] 
[01:01:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:00] 
[01:01:00] 
[01:01:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:00] Build completed unsuccessfully in 0:04:08
[01:01:00] Build completed unsuccessfully in 0:04:08
[01:01:00] make: *** [check] Error 1
[01:01:00] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b348624
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 09:59:51 UTC 2019
