\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/regions-mock-tcx.rs","byte_start":3625,"byte_end":3640,"line_start":137,"line_end":137,"column_start":21,"column_end":36,"is_primary":true,"text":[{"text":"    let ast_arena = TypedArena::new();","highlight_start":21,"highlight_end":36}],"label":"function or associated item not found in `arena::TypedArena<_>`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no function or associated item named `new` found for type `arena::TypedArena<_>` in the current scope\n  --> /checkout/src/test/run-pass-fulldeps/regions-mock-tcx.rs:137:21\n   |\nLL |     let ast_arena = TypedArena::new();\n   |                     ^^^^^^^^^^^^^^^ function or associated item not found in `arena::TypedArena<_>`\n\n"}
[01:05:51] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:05:51] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[01:05:51] ------------------------------------------
[01:05:51] 
[01:05:51] thread '[run-pass] run-pass-fulldeps/regions-mock-tcx.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:05:51] test result: FAILED. 96 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:05:51] 
[01:05:51] 
[01:05:51] 
[01:05:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:51] 
[01:05:51] 
[01:05:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:51] Build completed unsuccessfully in 0:21:51
[01:05:51] Build completed unsuccessfully in 0:21:51
[01:05:51] Makefile:58: recipe for target 'check' failed
[01:05:51] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07e3cd97
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1ecdaf6d:start=1539716860162919359,finish=1539716860167598230,duration=4678871
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2f015bd7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05f9614f
travis_time:start:05f9614f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:082e842c
$ dmesg | grep -i kill
