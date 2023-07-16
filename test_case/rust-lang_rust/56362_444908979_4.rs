\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/precise_pointer_size_matching.rs","byte_start":977,"byte_end":983,"line_start":32,"line_end":32,"column_start":11,"column_end":17,"is_primary":true,"text":[{"text":"    match 0usize { //~ ERROR non-exhaustive patterns","highlight_start":11,"highlight_end":17}],"label":"patterns `0usize` and `21usize..=18446744073709551615usize` not covered","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0004]: non-exhaustive patterns: `0usize` and `21usize..=18446744073709551615usize` not covered\n  --> /checkout/src/test/ui/precise_pointer_size_matching.rs:32:11\n   |\nLL |     match 0usize { //~ ERROR non-exhaustive patterns\n   |           ^^^^^^ patterns `0usize` and `21usize..=18446744073709551615usize` not covered\n\n"}
[00:46:39] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:39] {"message":"For more information about this error, try `rustc --explain E0004`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0004`.\n"}
[00:46:39] ------------------------------------------
[00:46:39] 
[00:46:39] thread '[ui] ui/precise_pointer_size_matching.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:46:39] 
---
[00:46:39] 
[00:46:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:46:39] 
[00:46:39] 
[00:46:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:39] 
[00:46:39] 
[00:46:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:39] Build completed unsuccessfully in 0:03:43
[00:46:39] Build completed unsuccessfully in 0:03:43
[00:46:39] Makefile:58: recipe for target 'check' failed
[00:46:39] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cb848aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec  6 15:24:05 UTC 2018
---
travis_time:end:028b5074:start=1544109846619441444,finish=1544109846623543734,duration=4102290
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1416a224
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!
