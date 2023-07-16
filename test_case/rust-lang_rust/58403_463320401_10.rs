\n\nStatics are shared everywhere, and if they refer to mutable data one might\nviolate memory safety since holding multiple mutable references to shared data\nis not allowed.\n\nIf you really want global mutable state, try using `static mut` or a global\n`UnsafeCell`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0388.rs","byte_start":340,"byte_end":346,"line_start":8,"line_end":8,"column_start":38,"column_end":44,"is_primary":true,"text":[{"text":"static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017","highlight_start":38,"highlight_end":44}],"label":"statics require immutable values","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0017]: references in statics may only refer to immutable values\n  --> /checkout/src/test/ui/error-codes/E0388.rs:8:38\n   |\nLL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017\n   |                                      ^^^^^^ statics require immutable values\n\n"}
[01:29:35] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:29:35] {"message":"Some errors occurred: E0017, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0017, E0596.\n"}
[01:29:35] 
[01:29:35] ------------------------------------------
[01:29:35] 
[01:29:35] thread '[ui (nll)] ui/error-codes/E0388.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:29:35] 
[01:29:35] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:29:35] 
[01:29:35] 
[01:29:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:29:35] 
[01:29:35] 
[01:29:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:35] Build completed unsuccessfully in 0:08:22
[01:29:35] Build completed unsuccessfully in 0:08:22
[01:29:35] Makefile:48: recipe for target 'check' failed
[01:29:35] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06b838f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 18:51:24 UTC 2019
---
travis_time:end:0a329c00:start=1550083886720277228,finish=1550083886730630758,duration=10353530
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:120e1c38
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a702fb6
travis_time:start:0a702fb6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bfff090
$ dmesg | grep -i kill
