\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/borrowck-ref-into-rvalue.rs","byte_start":502,"byte_end":527,"line_start":13,"line_end":13,"column_start":11,"column_end":36,"is_primary":true,"text":[{"text":"    match Some(\"Hello\".to_string()) {","highlight_start":11,"highlight_end":36}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/borrowck-ref-into-rvalue.rs","byte_start":680,"byte_end":681,"line_start":19,"line_end":19,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/borrowck-ref-into-rvalue.rs","byte_start":701,"byte_end":705,"line_start":20,"line_end":20,"column_start":20,"column_end":24,"is_primary":false,"text":[{"text":"    println!(\"{}\", *msg);","highlight_start":20,"highlight_end":24}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a `let` binding to create a longer lived value","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0714]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/span/borrowck-ref-into-rvalue.rs:13:11\n   |\nLL |     match Some(\"Hello\".to_string()) {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use\n...\nLL |     }\n   |     - temporary value is freed at the end of this statement\nLL |     println!(\"{}\", *msg);\n   |                    ---- borrow later used here\n   |\n   = note: consider using a `let` binding to create a longer lived value\n\n"}
[00:58:03] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:58:03] {"message":"For more information about this error, try `rustc --explain E0714`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0714`.\n"}
[00:58:03] ------------------------------------------
[00:58:03] 
[00:58:03] thread '[ui (nll)] ui/span/borrowck-ref-into-rvalue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:58:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:58:03] 
[00:58:03] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:58:03] 
[00:58:03] 
[00:58:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:58:03] 
[00:58:03] 
[00:58:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:03] Build completed unsuccessfully in 0:13:46
[00:58:03] Build completed unsuccessfully in 0:13:46
[00:58:03] Makefile:58: recipe for target 'check' failed
[00:58:03] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04c83124
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:12681d38:start=1537808522617549537,finish=1537808522621259350,duration=3709813
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:084a090b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:180bf48c
travis_time:start:180bf48c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f571afc
$ dmesg | grep -i kill
