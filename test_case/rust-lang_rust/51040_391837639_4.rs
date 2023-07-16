\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs","byte_start":1113,"byte_end":1165,"line_start":33,"line_end":33,"column_start":26,"column_end":78,"is_primary":true,"text":[{"text":"    let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();","highlight_start":26,"highlight_end":78}],"label":"temporary value does not live long enough","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs","byte_start":1167,"byte_end":1168,"line_start":34,"line_end":34,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value only lives until here","suggested_replacement":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough\n  --> /checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs:33:26\n   |\nLL |     let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough\nLL | }\n   | - temporary value only lives until here\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:47:53] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:47:53] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:47:53] ------------------------------------------
[00:47:53] 
[00:47:53] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:47:53] thread '[ui] ui/const-eval/dont_promote_unstable_const_fn.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
---
[00:47:53] test result: FAILED. 1380 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:47:53] 
[00:47:53] 
[00:47:53] 
[00:47:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:53] 
[00:47:53] 
[00:47:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:53] Build completed unsuccessfully in 0:02:30
[00:47:53] Build completed unsuccessfully in 0:02:30
[00:47:53] Makefile:58: recipe for target 'check' failed
[00:47:53] make: *** [check] Error 1
2728572 ./obj
2728540 ./obj/build
1965008 ./obj/build/x86_64-unknown-linux-gnu
927472 ./.git
---
149560 ./.git/modules
149556 ./.git/modules/src
149112 ./src/llvm-emscripten/test
123932 ./obj/build/bootstrap/debug/incremental/bootstrap-368kjpmz08h8o
123928 ./obj/build/bootstrap/debug/incremental/bootstrap-368kjpmz08h8o/s-f1cct4tn4t-shn094-a5eqdv1qeuis
112612 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
112608 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
108616 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
103440 ./obj/build/bootstrap/debug/incremental/bootstrap-152hben4kxal2
103440 ./obj/build/bootstrap/debug/incremental/bootstrap-152hben4kxal2
103436 ./obj/build/bootstrap/debug/incremental/bootstrap-152hben4kxal2/s-f1cdzfipci-13aqc0b-18d35k9c9hp7o
98488 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
93352 ./obj/build/x86_64-unknown-linux-gnu/stage1
93328 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90856 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90856 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90852 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f1cdwzhutq-z3j9ws-ek2w5d5pqxih
87768 ./obj/build/x86_64-unknown-linux-gnu/doc/core
81228 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
80932 ./obj/build/x86_64-unknown-linux-gnu/doc/std
79028 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
