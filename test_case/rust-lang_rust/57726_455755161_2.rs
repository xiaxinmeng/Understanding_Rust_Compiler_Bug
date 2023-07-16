\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":902,"byte_end":946,"line_start":34,"line_end":34,"column_start":1,"column_end":45,"is_primary":true,"text":[{"text":"impl LintPass for MissingWhitelistedAttrPass {","highlight_start":1,"highlight_end":45}],"label":"missing `name` in implementation","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`name` from trait: `fn(&Self) -> &'static str`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0046]: not all trait items implemented, missing: `name`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:34:1\n   |\nLL | impl LintPass for MissingWhitelistedAttrPass {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `name` in implementation\n   |\n   = note: `name` from trait: `fn(&Self) -> &'static str`\n\n"}
[01:21:30] {"message":"For more information about this error, try `rustc --explain E0046`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0046`.\n"}
[01:21:30] 
[01:21:30] ------------------------------------------
[01:21:30] 
---
[01:21:30] test result: FAILED. 59 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:21:30] 
[01:21:30] 
[01:21:30] 
[01:21:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:30] 
[01:21:30] 
[01:21:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:30] Build completed unsuccessfully in 0:17:38
[01:21:30] Build completed unsuccessfully in 0:17:38
[01:21:30] Makefile:48: recipe for target 'check' failed
[01:21:30] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1663ab30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 19 06:49:50 UTC 2019
---
175672 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
162608 ./obj/build/bootstrap/debug/incremental
153284 ./src/tools/clang
146492 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405
146488 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405/s-f8occr0lml-1k96bct-3sgufgpmw18gi
139060 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
139056 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
136716 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
124936 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
