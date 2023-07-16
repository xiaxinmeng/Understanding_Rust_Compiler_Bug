\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/yield-while-iterating.rs","byte_start":948,"byte_end":955,"line_start":23,"line_end":23,"column_start":13,"column_end":20,"is_primary":false,"text":[{"text":"            yield();","highlight_start":13,"highlight_end":20}],"label":"possible yield occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/yield-while-iterating.rs","byte_start":921,"byte_end":923,"line_start":22,"line_end":22,"column_start":18,"column_end":20,"is_primary":true,"text":[{"text":"        for p in &x { //~ ERROR","highlight_start":18,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0626]: borrow may still be in use when generator yields\n  --> /checkout/src/test/ui/generator/yield-while-iterating.rs:22:18\n   |\nLL |         for p in &x { //~ ERROR\n   |                  ^^\nLL |             yield();\n   |             ------- possible yield occurs here\n\n"}
[00:44:12] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:12] {"message":"For more information about this error, try `rustc --explain E0626`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0626`.\n"}
[00:44:12] ------------------------------------------
[00:44:12] 
[00:44:12] thread '[ui (nll)] ui/generator/yield-while-iterating.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:44:12] 
[00:44:12] 
[00:44:12] ---- [ui (nll)] ui/generator/yield-while-ref-reborrowed.rs stdout ----
[00:44:12]  
[00:44:12] error: ui test compiled successfully!
[00:44:12] status: exit code: 0
[00:44:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-while-ref-reborrowed.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-ref-reborrowed.stage2-x86_64-unknown-linux-gnu" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-ref-reborrowed.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:12] ------------------------------------------
[00:44:12] 
[00:44:12] ------------------------------------------
[00:44:12] stderr:
---
[00:44:12] 
[00:44:12] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:44:12] 
[00:44:12] 
[00:44:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:44:12] 
[00:44:12] 
[00:44:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:12] Build completed unsuccessfully in 0:03:44
[00:44:12] Build completed unsuccessfully in 0:03:44
[00:44:12] make: *** [check] Error 1
[00:44:12] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ff698c7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
