\n\nOnly structs and enums are permitted to impl Send, Sync, and other opt-out\ntrait, and the struct or enum must be local to the current crate. So, for\nexample, `unsafe impl Send for Rc<Foo>` is not allowed.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive.rs","byte_start":841,"byte_end":882,"line_start":27,"line_end":27,"column_start":1,"column_end":42,"is_primary":true,"text":[{"text":"unsafe impl Send for dyn Object + Marker2 {} //~ ERROR E0321","highlight_start":1,"highlight_end":42}],"label":"can't implement cross-crate trait with a default impl for non-struct/enum type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0321]: cross-crate traits with a default impl, like `std::marker::Send`, can only be implemented for a struct/enum type, not `(dyn Object + Marker2 + 'static)`\n  --> /checkout/src/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive.rs:27:1\n   |\nLL | unsafe impl Send for dyn Object + Marker2 {} //~ ERROR E0321\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type\n\n"}
[01:03:54] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:03:54] {"message":"Some errors occurred: E0117, E0321, E0371.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0117, E0321, E0371.\n"}
[01:03:54] 
[01:03:54] ------------------------------------------
[01:03:54] 
[01:03:54] thread '[ui] ui/coherence/coherence-impl-trait-for-marker-trait-positive.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:03:54] 
[01:03:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:03:54] 
[01:03:54] 
[01:03:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:54] 
[01:03:54] 
[01:03:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:54] Build completed unsuccessfully in 0:03:58
[01:03:54] Build completed unsuccessfully in 0:03:58
[01:03:54] Makefile:48: recipe for target 'check' failed
[01:03:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00ab8c33
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 13 16:16:36 UTC 2019
---
travis_time:end:0643b115:start=1547396197297393420,finish=1547396197303885202,duration=6491782
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f4165b6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:218c1c1d
$ dmesg | grep -i kill
