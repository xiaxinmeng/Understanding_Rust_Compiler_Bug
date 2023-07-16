plain
2019-11-15T23:05:55.1072335Z failures:
2019-11-15T23:05:55.1072602Z 
2019-11-15T23:05:55.1073275Z ---- [ui] ui/consts/issue-66397.rs stdout ----
2019-11-15T23:05:55.1073528Z 
2019-11-15T23:05:55.1074220Z error: test compilation failed although it shouldn't!
2019-11-15T23:05:55.1074540Z status: signal: 6
2019-11-15T23:05:55.1076136Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-66397.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-66397" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-66397/auxiliary" "-A" "unused"
2019-11-15T23:05:55.1077216Z ------------------------------------------
2019-11-15T23:05:55.1077446Z 
2019-11-15T23:05:55.1077883Z ------------------------------------------
2019-11-15T23:05:55.1078131Z stderr:
2019-11-15T23:05:55.1078131Z stderr:
2019-11-15T23:05:55.1078560Z ------------------------------------------
2019-11-15T23:05:55.1079310Z rustc: /checkout/src/llvm-project/llvm/lib/IR/Type.cpp:610: static llvm::VectorType* llvm::VectorType::get(llvm::Type*, llvm::ElementCount): Assertion `EC.Min > 0 && "#Elements of a VectorType must be greater than 0"' failed.
2019-11-15T23:05:55.1080034Z ------------------------------------------
2019-11-15T23:05:55.1080246Z 
2019-11-15T23:05:55.1080392Z 
2019-11-15T23:05:55.1080529Z 
---
2019-11-15T23:05:55.1112944Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-15T23:05:55.1113420Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-15T23:05:55.1119226Z 
2019-11-15T23:05:55.1119563Z 
2019-11-15T23:05:55.1128907Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-15T23:05:55.1129657Z 
2019-11-15T23:05:55.1129699Z 
2019-11-15T23:05:55.1138583Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-15T23:05:55.1139110Z Build completed unsuccessfully in 1:36:56
2019-11-15T23:05:55.1139110Z Build completed unsuccessfully in 1:36:56
2019-11-15T23:05:55.1194344Z == clock drift check ==
2019-11-15T23:05:55.8695167Z   local time: Fri Nov 15 23:05:55 UTC 2019
2019-11-15T23:05:55.8701716Z   network time: Fri, 15 Nov 2019 23:05:55 GMT
2019-11-15T23:05:55.8702132Z == end clock drift check ==
2019-11-15T23:05:56.6044306Z 
2019-11-15T23:05:56.6121425Z ##[error]Bash exited with code '1'.
2019-11-15T23:05:56.6200776Z ##[section]Starting: Checkout
2019-11-15T23:05:56.6203187Z ==============================================================================
2019-11-15T23:05:56.6203310Z Task         : Get sources
2019-11-15T23:05:56.6203410Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
