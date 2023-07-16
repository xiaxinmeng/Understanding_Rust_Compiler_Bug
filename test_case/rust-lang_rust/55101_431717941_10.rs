\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias.rs","byte_start":1124,"byte_end":1127,"line_start":36,"line_end":36,"column_start":16,"column_end":19,"is_primary":true,"text":[{"text":"    let both = foo();","highlight_start":16,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias.rs","byte_start":996,"byte_end":1021,"line_start":29,"line_end":29,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"fn foo<T: CD>() -> (T, T) {","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0283]: type annotations required: cannot resolve `_: CD`\n  --> /checkout/src/test/ui/traits/trait-alias.rs:36:16\n   |\nLL |     let both = foo();\n   |                ^^^\n   |\nnote: required by `foo`\n  --> /checkout/src/test/ui/traits/trait-alias.rs:29:1\n   |\nLL | fn foo<T: CD>() -> (T, T) {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:16] {"message":"For more information about this error, try `rustc --explain E0283`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0283`.\n"}
[00:49:16] ------------------------------------------
[00:49:16] 
[00:49:16] thread '[ui] ui/traits/trait-alias.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:49:16] 
---
[00:49:16] test result: FAILED. 4908 passed; 2 failed; 22 ignored; 0 measured; 0 filtered out
[00:49:16] 
[00:49:16] 
[00:49:16] 
[00:49:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:16] 
[00:49:16] 
[00:49:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:16] Build completed unsuccessfully in 0:03:37
[00:49:16] Build completed unsuccessfully in 0:03:37
[00:49:16] Makefile:58: recipe for target 'check' failed
[00:49:16] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05755464
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08003ab4:start=1540168742865591397,finish=1540168742871621860,duration=6030463
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00522668
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ba92aa8
travis_time:start:0ba92aa8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09fa2ffa
$ dmesg | grep -i kill
