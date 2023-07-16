plain
[00:44:06] ....................................................................................................
[00:44:11] .....................................................................................i..............
[00:44:16] ..............................................................i.....................................
[00:44:20] ....................................................................................................
[00:44:26] ......................F.............................................................................
[00:44:32] ...........................................................................................i......F.
":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/token/issue-10636-2.rs","byte_start":763,"byte_end":764,"line_start":18,"line_end":18,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"} //~ ERROR: incorrect close delimiter","highlight_start":1,"highlight_end":2}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `)`\n  --> /checkout/src/test/ui/token/issue-10636-2.rs:18:1\n   |\nLL | } //~ ERROR: incorrect close delimiter\n   | ^ expected expression\n\n"}
[00:44:34] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:44:34] ------------------------------------------
[00:44:34] 
[00:44:34] thread '[ui] ui/token/issue-10636-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:44:34] 
---
[00:44:34] test result: FAILED. 1453 passed; 2 failed; 14 ignored; 0 measured; 0 filtered out
[00:44:34] 
[00:44:34] 
[00:44:34] 
[00:44:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:34] 
[00:44:34] 
[00:44:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:34] Build completed unsuccessfully in 0:02:30
[00:44:34] Build completed unsuccessfully in 0:02:30
[00:44:34] Makefile:58: recipe for target 'check' failed
[00:44:34] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2358a81d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
