\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-51714.rs","byte_start":279,"byte_end":316,"line_start":11,"line_end":11,"column_start":10,"column_end":47,"is_primary":true,"text":[{"text":"    [(); return while let Some(n) = Some(0) {}];","highlight_start":10,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0572]: return statement outside of function body\n  --> /checkout/src/test/ui/issues/issue-51714.rs:11:10\n   |\nLL |     [(); return while let Some(n) = Some(0) {}];\n   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:08:38] {"message":"For more information about this error, try `rustc --explain E0572`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0572`.\n"}
[01:08:38] 
[01:08:38] ------------------------------------------
[01:08:38] 
---
[01:08:38] 
[01:08:38] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:08:38] 
[01:08:38] 
[01:08:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:38] 
[01:08:38] 
[01:08:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:38] Build completed unsuccessfully in 0:04:29
[01:08:38] Build completed unsuccessfully in 0:04:29
[01:08:38] make: *** [check] Error 1
[01:08:38] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1867ebb2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr  7 20:27:06 UTC 2019
---
travis_time:end:008d9e27:start=1554668828251752419,finish=1554668828259451904,duration=7699485
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2378130a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0767374e
$ dmesg | grep -i kill
