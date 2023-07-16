\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs","byte_start":576,"byte_end":582,"line_start":18,"line_end":18,"column_start":7,"column_end":13,"is_primary":true,"text":[{"text":"    g(&mut b) //~ ERROR cannot borrow","highlight_start":7,"highlight_end":13}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0596]: cannot borrow immutable item `b` as mutable\n  --> /checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs:18:7\n   |\nLL |     g(&mut b) //~ ERROR cannot borrow\n   |       ^^^^^^ cannot borrow as mutable\n\n"}
[00:50:13] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:13] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:50:13] ------------------------------------------
[00:50:13] 
[00:50:13] thread '[ui (nll)] ui/borrowck/mut-borrow-of-mut-ref.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:50:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:13] 
[00:50:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:13] 
[00:50:13] 
[00:50:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:50:13] 
[00:50:13] 
[00:50:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:13] Build completed unsuccessfully in 0:03:55
[00:50:13] Build completed unsuccessfully in 0:03:55
[00:50:13] make: *** [check] Error 1
[00:50:13] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07553010
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
