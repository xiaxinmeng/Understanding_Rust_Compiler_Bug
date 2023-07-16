\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/underscore_const_names_feature_gate.rs","byte_start":512,"byte_end":530,"line_start":12,"line_end":12,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"static _: () = (); //~ ERROR is unstable","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(underscore_const_names)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: naming constants with `_` is unstable (see issue #54912)\n  --> /checkout/src/test/ui/underscore_const_names_feature_gate.rs:12:1\n   |\nLL | static _: () = (); //~ ERROR is unstable\n   | ^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(underscore_const_names)] to the crate attributes to enable\n\n"}
[00:51:56] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:56] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:51:56] ------------------------------------------
[00:51:56] 
[00:51:56] thread '[ui] ui/underscore_const_names_feature_gate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:51:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:56] 
[00:51:56] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:51:56] 
[00:51:56] 
[00:51:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:56] 
[00:51:56] 
[00:51:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:56] Build completed unsuccessfully in 0:03:49
[00:51:56] Build completed unsuccessfully in 0:03:49
[00:51:56] Makefile:58: recipe for target 'check' failed
[00:51:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19699598
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 15 16:12:15 UTC 2018
---
travis_time:end:01a3dea6:start=1542298336537033170,finish=1542298336542549093,duration=5515923
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00b5a978
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog
