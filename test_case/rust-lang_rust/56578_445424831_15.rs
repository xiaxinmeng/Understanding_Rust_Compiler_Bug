\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs","byte_start":1113,"byte_end":1165,"line_start":33,"line_end":33,"column_start":26,"column_end":78,"is_primary":true,"text":[{"text":"    let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();","highlight_start":26,"highlight_end":78}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs","byte_start":1208,"byte_end":1209,"line_start":35,"line_end":35,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs","byte_start":1099,"byte_end":1109,"line_start":33,"line_end":33,"column_start":12,"column_end":22,"is_primary":false,"text":[{"text":"    let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();","highlight_start":12,"highlight_end":22}],"label":"type annotation requires that borrow lasts for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs:33:26\n   |\nLL |     let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();\n   |            ----------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use\n   |            |\n   |            type annotation requires that borrow lasts for `'static`\nLL |     //~^ ERROR does not live long enough\nLL | }\n   | - temporary value is freed at the end of this statement\n\n"}
[01:08:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:08:49] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:08:49] make: *** [check] Error 1
[01:08:49] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[01:08:49] ------------------------------------------
[01:08:49] 
[01:08:49] thread '[ui (nll)] ui/consts/const-eval/dont_promote_unstable_const_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:08:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:08:49] test result: FAILED. 5041 passed; 1 failed; 85 ignored; 0 measured; 0 filtered out
[01:08:49] 
[01:08:49] 
[01:08:49] 
[01:08:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:08:49] 
[01:08:49] 
[01:08:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:49] Build completed unsuccessfully in 0:07:38
[01:08:49] Build completed unsuccessfully in 0:07:38
[01:08:49] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:206242d7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 02:58:14 UTC 2018
---
travis_time:end:07d715c0:start=1544237896506891224,finish=1544237896514299367,duration=7408143
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06ace6d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:015d1158
travis_time:start:015d1158
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:032c1c6e
$ dmesg | grep -i kill
