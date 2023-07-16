\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-async-await-2015-edition.rs","byte_start":17,"byte_end":34,"line_start":3,"line_end":3,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"async fn foo() {} //~ ERROR `async fn` is not permitted in the 2015 edition","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/50547","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add #![feature(async_await)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: async fn is unstable\n  --> /checkout/src/test/ui/feature-gates/feature-gate-async-await-2015-edition.rs:3:1\n   |\nLL | async fn foo() {} //~ ERROR `async fn` is not permitted in the 2015 edition\n   | ^^^^^^^^^^^^^^^^^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/50547\n   = help: add #![feature(async_await)] to the crate attributes to enable\n\n"}
[01:07:03] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:07:03] {"message":"Some errors occurred: E0422, E0425, E0658, E0670.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0422, E0425, E0658, E0670.\n"}
[01:07:03] 
[01:07:03] ------------------------------------------
[01:07:03] 
[01:07:03] thread '[ui] ui/feature-gates/feature-gate-async-await-2015-edition.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3469:9
---
[01:07:03] 
[01:07:03] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:07:03] 
[01:07:03] 
[01:07:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:03] 
[01:07:03] 
[01:07:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:03] Build completed unsuccessfully in 0:04:18
[01:07:03] Build completed unsuccessfully in 0:04:18
[01:07:03] make: *** [check] Error 1
[01:07:03] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003ba3d6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 16 02:51:15 UTC 2019
