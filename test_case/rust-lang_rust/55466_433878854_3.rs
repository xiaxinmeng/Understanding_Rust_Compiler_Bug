\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs","byte_start":879,"byte_end":882,"line_start":28,"line_end":28,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"use syntax::str::char_at;","highlight_start":13,"highlight_end":16}],"label":"Could not find `str` in `syntax`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved import `syntax::str`\n  --> /checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs:28:13\n   |\nLL | use syntax::str::char_at;\n   |             ^^^ Could not find `str` in `syntax`\n\n"}
[01:11:12] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:11:12] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:11:12] ------------------------------------------
[01:11:12] 
[01:11:12] thread '[run-pass] run-pass-fulldeps/ast_stmt_expr_attr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:11:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:11:12] test result: FAILED. 96 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:11:12] 
[01:11:12] 
[01:11:12] 
[01:11:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:12] 
[01:11:12] 
[01:11:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:12] Build completed unsuccessfully in 0:24:21
[01:11:12] Build completed unsuccessfully in 0:24:21
[01:11:12] make: *** [check] Error 1
[01:11:12] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:23d3fff2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:077d63f3:start=1540813020532814478,finish=1540813020539681363,duration=6866885
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02b08d52
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c8953c4
travis_time:start:0c8953c4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b010b32
$ dmesg | grep -i kill
