\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs","byte_start":1206,"byte_end":1210,"line_start":39,"line_end":39,"column_start":20,"column_end":24,"is_primary":true,"text":[{"text":"        match &*it.name.as_str() {","highlight_start":20,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0609]: no field `name` on type `&rustc::hir::Item`\n  --> /checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs:39:20\n   |\nLL |         match &*it.name.as_str() {\n   |                    ^^^^\n\n"}
[00:56:22] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:22] {"message":"For more information about this error, try `rustc --explain E0609`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0609`.\n"}
[00:56:22] ------------------------------------------
[00:56:22] 
[00:56:22] thread '[ui] ui-fulldeps/lint-group-plugin.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:56:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:22] 
[00:56:22] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:56:22] 
[00:56:22] 
[00:56:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:22] 
[00:56:22] 
[00:56:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:22] Build completed unsuccessfully in 0:11:33
[00:56:22] Build completed unsuccessfully in 0:11:33
[00:56:22] Makefile:58: recipe for target 'check' failed
[00:56:22] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2c0c2a04
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 22:51:56 UTC 2018
---
travis_time:end:0b83de70:start=1543877517791764063,finish=1543877517796973422,duration=5209359
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2dae3d14
$ ln -s . checkout && f
