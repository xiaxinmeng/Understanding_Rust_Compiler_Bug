\n\nRemember this solution is unsafe! You will have to ensure that accesses to the\ncell are synchronized.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unsafe/ranged_ints3_const.rs","byte_start":576,"byte_end":580,"line_start":19,"line_end":19,"column_start":22,"column_end":26,"is_primary":true,"text":[{"text":"    let y = unsafe { &x.0 }; //~ ERROR cannot borrow a constant which may contain interior mut","highlight_start":22,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead\n  --> /checkout/src/test/ui/unsafe/ranged_ints3_const.rs:19:22\n   |\nLL |     let y = unsafe { &x.0 }; //~ ERROR cannot borrow a constant which may contain interior mut\n   |                      ^^^^\n\n"}
[01:00:53] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:00:53] {"message":"Some errors occurred: E0133, E0492.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0133, E0492.\n"}
[01:00:53] 
[01:00:53] ------------------------------------------
[01:00:53] 
[01:00:53] thread '[ui] ui/unsafe/ranged_ints3_const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3287:9
---
[01:00:53] 
[01:00:53] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:00:53] 
[01:00:53] 
[01:00:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:53] 
[01:00:53] 
[01:00:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:53] Build completed unsuccessfully in 0:04:15
[01:00:53] Build completed unsuccessfully in 0:04:15
[01:00:53] make: *** [check] Error 1
[01:00:53] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09b421c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 26 16:16:37 UTC 2019
