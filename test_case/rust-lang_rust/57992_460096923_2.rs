\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/futures-api.rs","byte_start":1634,"byte_end":1638,"line_start":72,"line_end":72,"column_start":6,"column_end":10,"is_primary":true,"text":[{"text":"impl Wake for Counter {","highlight_start":6,"highlight_end":10}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0405]: cannot find trait `Wake` in this scope\n  --> /checkout/src/test/run-pass/futures-api.rs:72:6\n   |\nLL | impl Wake for Counter {\n   |      ^^^^ not found in this scope\n\n"}
[01:06:32] {"message":"For more information about this error, try `rustc --explain E0405`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0405`.\n"}
[01:06:32] 
[01:06:32] ------------------------------------------
[01:06:32] 
---
[01:06:32] 
[01:06:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:06:32] 
[01:06:32] 
[01:06:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:32] 
[01:06:32] 
[01:06:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:32] Build completed unsuccessfully in 0:10:45
[01:06:32] Build completed unsuccessfully in 0:10:45
[01:06:32] Makefile:48: recipe for target 'check' failed
[01:06:32] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:062f4139
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 22:55:11 UTC 2019
