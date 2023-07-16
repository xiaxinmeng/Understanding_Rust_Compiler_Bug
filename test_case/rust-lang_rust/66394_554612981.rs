plain
2019-11-16T07:25:51.9815436Z failures:
2019-11-16T07:25:51.9838742Z 
2019-11-16T07:25:51.9839155Z ---- [ui] ui/consts/issue-66397.rs stdout ----
2019-11-16T07:25:51.9839220Z 
2019-11-16T07:25:51.9839434Z error: test compilation failed although it shouldn't!
2019-11-16T07:25:51.9839516Z status: signal: 6
2019-11-16T07:25:51.9841082Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-66397.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-66397" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "--emit" "mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-66397/auxiliary" "-A" "unused"
2019-11-16T07:25:51.9841716Z ------------------------------------------
2019-11-16T07:25:51.9841771Z 
2019-11-16T07:25:51.9842044Z ------------------------------------------
2019-11-16T07:25:51.9842120Z stderr:
2019-11-16T07:25:51.9842120Z stderr:
2019-11-16T07:25:51.9842369Z ------------------------------------------
2019-11-16T07:25:51.9843064Z rustc: /checkout/src/llvm-project/llvm/lib/IR/Type.cpp:610: static llvm::VectorType* llvm::VectorType::get(llvm::Type*, llvm::ElementCount): Assertion `EC.Min > 0 && "#Elements of a VectorType must be greater than 0"' failed.
2019-11-16T07:25:51.9843393Z ------------------------------------------
2019-11-16T07:25:51.9843434Z 
2019-11-16T07:25:51.9843463Z 
2019-11-16T07:25:51.9843491Z 
---
2019-11-16T07:25:51.9869907Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-16T07:25:51.9870198Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-16T07:25:51.9882032Z 
2019-11-16T07:25:51.9882129Z 
2019-11-16T07:25:51.9883942Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--linker" "x86_64-linux-musl-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-16T07:25:51.9884736Z 
2019-11-16T07:25:51.9884786Z 
2019-11-16T07:25:51.9890787Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
2019-11-16T07:25:51.9890909Z Build completed unsuccessfully in 0:16:18
2019-11-16T07:25:51.9890909Z Build completed unsuccessfully in 0:16:18
2019-11-16T07:25:51.9946130Z == clock drift check ==
2019-11-16T07:25:51.9965431Z   local time: Sat Nov 16 07:25:51 UTC 2019
2019-11-16T07:25:52.2184244Z   network time: Sat, 16 Nov 2019 07:25:52 GMT
2019-11-16T07:25:52.2184809Z == end clock drift check ==
2019-11-16T07:25:53.6780528Z 
2019-11-16T07:25:53.6871415Z ##[error]Bash exited with code '1'.
2019-11-16T07:25:53.6931455Z ##[section]Starting: Checkout
2019-11-16T07:25:53.6933478Z ==============================================================================
2019-11-16T07:25:53.6933558Z Task         : Get sources
2019-11-16T07:25:53.6933648Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
