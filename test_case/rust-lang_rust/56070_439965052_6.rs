\n\nStatics are shared everywhere, and if they refer to mutable data one might\nviolate memory safety since holding multiple mutable references to shared data\nis not allowed.\n\nIf you really want global mutable state, try using `static mut` or a global\n`UnsafeCell`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0388.rs","byte_start":807,"byte_end":813,"line_start":18,"line_end":18,"column_start":38,"column_end":44,"is_primary":true,"text":[{"text":"static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017","highlight_start":38,"highlight_end":44}],"label":"statics require immutable values","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0017]: references in statics may only refer to immutable values\n  --> /checkout/src/test/ui/error-codes/E0388.rs:18:38\n   |\nLL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017\n   |                                      ^^^^^^ statics require immutable values\n\n"}
[00:51:54] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:51:54] {"message":"Some errors occurred: E0017, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0017, E0596.\n"}
[00:51:54] {"message":"For more information about an error, try `rustc --explain E0017`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0017`.\n"}
[00:51:54] ------------------------------------------
[00:51:54] 
[00:51:54] thread '[ui (nll)] ui/error-codes/E0388.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:51:54] 
---
[00:51:54] test result: FAILED. 4953 passed; 2 failed; 86 ignored; 0 measured; 0 filtered out
[00:51:54] 
[00:51:54] 
[00:51:54] 
[00:51:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:51:54] 
[00:51:54] 
[00:51:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:54] Build completed unsuccessfully in 0:06:11
[00:51:54] Build completed unsuccessfully in 0:06:11
[00:51:54] Makefile:58: recipe for target 'check' failed
[00:51:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:005199f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 19 16:58:10 UTC 2018
