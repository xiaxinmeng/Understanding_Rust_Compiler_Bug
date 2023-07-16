\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs","byte_start":498,"byte_end":520,"line_start":20,"line_end":20,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"impl LintPass for Pass {","highlight_start":1,"highlight_end":23}],"label":"missing `name` in implementation","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`name` from trait: `fn(&Self) -> &'static str`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0046]: not all trait items implemented, missing: `name`\n  --> /checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs:20:1\n   |\nLL | impl LintPass for Pass {\n   | ^^^^^^^^^^^^^^^^^^^^^^ missing `name` in implementation\n   |\n   = note: `name` from trait: `fn(&Self) -> &'static str`\n\n"}
[01:14:00] {"message":"For more information about this error, try `rustc --explain E0046`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0046`.\n"}
[01:14:00] 
[01:14:00] ------------------------------------------
[01:14:00] 
---
[01:14:00] test result: FAILED. 13 passed; 12 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:00] 
[01:14:00] 
[01:14:00] 
[01:14:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:00] 
[01:14:00] 
[01:14:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:00] Build completed unsuccessfully in 0:13:33
[01:14:00] Build completed unsuccessfully in 0:13:33
[01:14:00] Makefile:48: recipe for target 'check' failed
[01:14:00] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e533411
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 19 05:17:08 UTC 2019
