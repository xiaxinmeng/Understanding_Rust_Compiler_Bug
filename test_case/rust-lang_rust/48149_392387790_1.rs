\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0131.rs","byte_start":474,"byte_end":477,"line_start":11,"line_end":11,"column_start":8,"column_end":11,"is_primary":true,"text":[{"text":"fn main<T>() {","highlight_start":8,"highlight_end":11}],"label":"`main` cannot have generic parameters","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0131]: `main` function is not allowed to have generic parameters\n  --> /checkout/src/test/ui/error-codes/E0131.rs:11:8\n   |\nLL | fn main<T>() {\n   |        ^^^ `main` cannot have generic parameters\n\n"}
[00:47:49] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:49] {"message":"For more information about this error, try `rustc --explain E0131`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0131`.\n"}
[00:47:49] ------------------------------------------
[00:47:49] 
[00:47:49] thread '[ui] ui/error-codes/E0131.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:47:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:49] test result: FAILED. 1455 passed; 1 failed; 14 ignored; 0 measured; 0 filtered out
[00:47:49] 
[00:47:49] 
[00:47:49] 
[00:47:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:49] 
[00:47:49] 
[00:47:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:49] Build completed unsuccessfully in 0:02:36
[00:47:49] Build completed unsuccessfully in 0:02:36
[00:47:49] Makefile:58: recipe for target 'check' failed
[00:47:49] make: *** [check] Error 1
60840 ./src/llvm-emscripten/lib
59864 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
56536 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
56532 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
