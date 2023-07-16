\n\nMust always be called with exactly two arguments, e.g. `f(2, \"test\")`.\n\nNote that Rust does not have a notion of optional function arguments or\nvariadic functions (except for its C-FFI).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/binding/fn-arg-incomplete-pattern-drop-order.rs","byte_start":1929,"byte_end":1937,"line_start":68,"line_end":68,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"    (0..=4).for_each(test_drop_order, bindings_in_closure);","highlight_start":13,"highlight_end":21}],"label":"expected 1 parameter","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0061]: this function takes 1 parameter but 2 parameters were supplied\n  --> /checkout/src/test/run-pass/binding/fn-arg-incomplete-pattern-drop-order.rs:68:13\n   |\nLL |     (0..=4).for_each(test_drop_order, bindings_in_closure);\n   |             ^^^^^^^^ expected 1 parameter\n\n"}
[00:56:47] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:56:47] {"message":"For more information about this error, try `rustc --explain E0061`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0061`.\n"}
[00:56:47] ------------------------------------------
[00:56:47] 
[00:56:47] thread '[run-pass] run-pass/binding/fn-arg-incomplete-pattern-drop-order.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:56:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:47] 
[00:56:47] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:56:47] 
[00:56:47] 
[00:56:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:47] 
[00:56:47] 
[00:56:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:47] Build completed unsuccessfully in 0:12:10
[00:56:47] Build completed unsuccessfully in 0:12:10
[00:56:47] Makefile:58: recipe for target 'check' failed
[00:56:47] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2534ae80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 29 22:22:01 UTC 2018
---
travis_time:end:10365f2e:start=1543530123096078110,finish=1543530123156577087,duration=60498977
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1003a5ac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:255ab500
$ dmesg | grep -i kill
