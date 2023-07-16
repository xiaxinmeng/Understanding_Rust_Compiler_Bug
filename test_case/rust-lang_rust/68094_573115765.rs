plain
2020-01-10T16:50:56.2961256Z ---- [ui] ui/macros/issue-68058.rs stdout ----
2020-01-10T16:50:56.2961321Z 
2020-01-10T16:50:56.2961739Z error: test compilation failed although it shouldn't!
2020-01-10T16:50:56.2961924Z status: exit code: 1
2020-01-10T16:50:56.2962749Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-68058.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68058" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68058/auxiliary" "-A" "unused"
2020-01-10T16:50:56.2963418Z ------------------------------------------
2020-01-10T16:50:56.2963461Z 
2020-01-10T16:50:56.2963813Z ------------------------------------------
2020-01-10T16:50:56.2963981Z stderr:
2020-01-10T16:50:56.2963981Z stderr:
2020-01-10T16:50:56.2964306Z ------------------------------------------
2020-01-10T16:50:56.2964481Z error: the feature named `avx2` is not valid for this target
2020-01-10T16:50:56.2964786Z   --> /checkout/src/test/ui/macros/issue-68058.rs:5:26
2020-01-10T16:50:56.2964975Z    |
2020-01-10T16:50:56.2965084Z LL |         #[target_feature(enable=$target)]
2020-01-10T16:50:56.2965165Z    |                          ^^^^^^^^^^^^^^ `avx2` is not valid for this target
2020-01-10T16:50:56.2965321Z ...
2020-01-10T16:50:56.2965459Z LL | def_target!("avx2");
2020-01-10T16:50:56.2965857Z 
2020-01-10T16:50:56.2966007Z error: aborting due to previous error
2020-01-10T16:50:56.2966091Z 
2020-01-10T16:50:56.2966121Z 
---
2020-01-10T16:50:56.2993055Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-10T16:50:56.2993153Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T16:50:56.3004982Z 
2020-01-10T16:50:56.3005910Z 
2020-01-10T16:50:56.3010465Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T16:50:56.3011320Z 
2020-01-10T16:50:56.3011374Z 
2020-01-10T16:50:56.3020291Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-01-10T16:50:57.1434987Z Build completed unsuccessfully in 1:03:59
2020-01-10T16:50:57.1434987Z Build completed unsuccessfully in 1:03:59
2020-01-10T16:50:57.1435662Z == clock drift check ==
2020-01-10T16:50:57.1435858Z   local time: Fri Jan 10 16:50:56 UTC 2020
2020-01-10T16:50:57.1436002Z   network time: Fri, 10 Jan 2020 16:50:56 GMT
2020-01-10T16:50:57.1436148Z == end clock drift check ==
2020-01-10T16:50:57.2604043Z 
2020-01-10T16:50:57.2683239Z ##[error]Bash exited with code '1'.
2020-01-10T16:50:57.2726622Z ##[section]Starting: Checkout
2020-01-10T16:50:57.2728175Z ==============================================================================
2020-01-10T16:50:57.2728262Z Task         : Get sources
2020-01-10T16:50:57.2728324Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
