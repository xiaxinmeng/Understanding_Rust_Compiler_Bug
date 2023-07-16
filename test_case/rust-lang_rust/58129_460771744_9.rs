\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/panic-uninitialized-zeroed.rs","byte_start":2851,"byte_end":2861,"line_start":93,"line_end":93,"column_start":58,"column_end":68,"is_primary":true,"text":[{"text":"                mem::MaybeUninit::<Bar>::uninitialized().into_inner()","highlight_start":58,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0624]: method `into_inner` is private\n  --> /checkout/src/test/run-pass/panic-uninitialized-zeroed.rs:93:58\n   |\nLL |                 mem::MaybeUninit::<Bar>::uninitialized().into_inner()\n   |                                                          ^^^^^^^^^^\n\n"}
[01:12:11] {"message":"For more information about this error, try `rustc --explain E0624`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0624`.\n"}
[01:12:11] 
[01:12:11] ------------------------------------------
[01:12:11] 
---
[01:12:11] 
[01:12:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:12:11] 
[01:12:11] 
[01:12:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:11] 
[01:12:11] 
[01:12:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:11] Build completed unsuccessfully in 0:11:05
[01:12:11] Build completed unsuccessfully in 0:11:05
[01:12:11] make: *** [check] Error 1
[01:12:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0501d416
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 19:30:19 UTC 2019
---
travis_time:end:17876aba:start=1549395020322579007,finish=1549395020327817463,duration=5238456
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00115b5c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's
