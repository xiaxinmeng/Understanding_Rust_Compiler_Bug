\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-36082.rs","byte_start":656,"byte_end":666,"line_start":21,"line_end":21,"column_start":19,"column_end":29,"is_primary":true,"text":[{"text":"    let val: &_ = x.borrow().0;","highlight_start":19,"highlight_end":29}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-36082.rs","byte_start":668,"byte_end":669,"line_start":21,"line_end":21,"column_start":31,"column_end":32,"is_primary":false,"text":[{"text":"    let val: &_ = x.borrow().0;","highlight_start":31,"highlight_end":32}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-36082.rs","byte_start":1261,"byte_end":1264,"line_start":30,"line_end":30,"column_start":20,"column_end":23,"is_primary":false,"text":[{"text":"    println!(\"{}\", val);","highlight_start":20,"highlight_end":23}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a `let` binding to create a longer lived value","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/issues/issue-36082.rs:21:19\n   |\nLL |     let val: &_ = x.borrow().0;\n   |                   ^^^^^^^^^^  - temporary value is freed at the end of this statement\n   |                   |\n   |                   creates a temporary which is freed while still in use\n...\nLL |     println!(\"{}\", val);\n   |                    --- borrow later used here\n   |\n   = note: consider using a `let` binding to create a longer lived value\n\n"}
[01:19:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:16] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[01:19:16] ------------------------------------------
[01:19:16] 
[01:19:16] thread '[ui (nll)] ui/issues/issue-36082.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:19:16] 
---
[01:19:16] test result: FAILED. 5104 passed; 2 failed; 88 ignored; 0 measured; 0 filtered out
[01:19:16] 
[01:19:16] 
[01:19:16] 
[01:19:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:19:16] 
[01:19:16] 
[01:19:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:16] Build completed unsuccessfully in 0:07:07
[01:19:16] Build completed unsuccessfully in 0:07:07
[01:19:16] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12a80b7d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec 22 17:45:23 UTC 2018
---
travis_time:end:08e6dac0:start=1545500725168600980,finish=1545500725174712459,duration=6111479
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07e601d9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08bba2a8
travis_time:start:08bba2a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ae0ef16
$ dmesg | grep -i kill
