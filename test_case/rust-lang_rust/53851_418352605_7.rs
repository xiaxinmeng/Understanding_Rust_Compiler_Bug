\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/promotion.rs","byte_start":639,"byte_end":645,"line_start":18,"line_end":18,"column_start":42,"column_end":48,"is_primary":true,"text":[{"text":"    let a: &'static Option<Cell<i32>> = &foo6(); // doesn't error on HIR borrowck","highlight_start":42,"highlight_end":48}],"label":"temporary value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/min_const_fn/promotion.rs","byte_start":680,"byte_end":681,"line_start":19,"line_end":19,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value only lives until here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough\n  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:18:42\n   |\nLL |     let a: &'static Option<Cell<i32>> = &foo6(); // doesn't error on HIR borrowck\n   |                                          ^^^^^^ temporary value does not live long enough\nLL | }\n   | - temporary value only lives until here\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:48:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:48:58] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:48:58] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:48:58] ------------------------------------------
[00:48:58] 
[00:48:58] thread '[ui (nll)] ui/consts/min_const_fn/promotion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:48:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:58] test result: FAILED. 4130 passed; 1 failed; 64 ignored; 0 measured; 0 filtered out
[00:48:58] 
[00:48:58] 
[00:48:58] 
[00:48:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:48:58] 
[00:48:58] 
[00:48:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:58] Build completed unsuccessfully in 0:05:03
[00:48:58] Build completed unsuccessfully in 0:05:03
[00:48:58] Makefile:58: recipe for target 'check' failed
[00:48:58] make: *** [check] Error 1
129812 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
129808 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
128660 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128656 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
