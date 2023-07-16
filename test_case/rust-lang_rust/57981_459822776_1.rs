\n\nIf you don't know the basics of Rust, you can go look to the Rust Book to get\nstarted: https://doc.rust-lang.org/book/\n"},"level":"error","spans":[],"children":[{"message":"consider adding a `main` function to `/checkout/src/test/ui/issues/issue-57979.rs`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0601]: `main` function not found in crate `issue_57979`\n   |\n   = note: consider adding a `main` function to `/checkout/src/test/ui/issues/issue-57979.rs`\n\n"}
[01:01:07] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:01:07] {"message":"Some errors occurred: E0601, E0666.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0601, E0666.\n"}
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] thread '[ui] ui/issues/issue-57979.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:01:07] 
[01:01:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:01:07] 
[01:01:07] 
[01:01:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:07] 
[01:01:07] 
[01:01:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:07] Build completed unsuccessfully in 0:04:05
[01:01:07] Build completed unsuccessfully in 0:04:05
[01:01:07] make: *** [check] Error 1
[01:01:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16973fae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 18:34:26 UTC 2019
---
travis_time:end:257adab8:start=1549046067773508098,finish=1549046067778527594,duration=5019496
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e7cff50
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!chec
