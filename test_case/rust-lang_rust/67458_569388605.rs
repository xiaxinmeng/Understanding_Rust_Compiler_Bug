plain
2019-12-28T05:46:07.9896014Z ---- [ui] ui/non-ice-error-on-worker-io-fail.rs stdout ----
2019-12-28T05:46:07.9896097Z 
2019-12-28T05:46:07.9896160Z error: ui test compiled successfully!
2019-12-28T05:46:07.9896238Z status: exit code: 0
2019-12-28T05:46:07.9897165Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/non-ice-error-on-worker-io-fail.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-ice-error-on-worker-io-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-o" "/dev/null" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-ice-error-on-worker-io-fail/auxiliary" "-A" "unused"
2019-12-28T05:46:07.9897689Z ------------------------------------------
2019-12-28T05:46:07.9897735Z 
2019-12-28T05:46:07.9897967Z ------------------------------------------
2019-12-28T05:46:07.9898031Z stderr:
2019-12-28T05:46:07.9898031Z stderr:
2019-12-28T05:46:07.9898254Z ------------------------------------------
2019-12-28T05:46:07.9898474Z warning: ignoring --out-dir flag due to -o flag
2019-12-28T05:46:07.9898564Z 
2019-12-28T05:46:07.9898786Z ------------------------------------------
2019-12-28T05:46:07.9898827Z 
2019-12-28T05:46:07.9898857Z 
---
2019-12-28T05:46:07.9932379Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-28T05:46:07.9932692Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-28T05:46:07.9952306Z 
2019-12-28T05:46:07.9953923Z 
2019-12-28T05:46:07.9957396Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-emscripten" "--mode" "ui" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-28T05:46:07.9961958Z 
2019-12-28T05:46:07.9962071Z 
2019-12-28T05:46:07.9968942Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten --exclude src/libcore --exclude src/liballoc --exclude src/libproc_macro --exclude src/libstd --exclude src/libterm --exclude src/libtest
2019-12-28T05:46:07.9969126Z Build completed unsuccessfully in 2:02:00
2019-12-28T05:46:07.9969126Z Build completed unsuccessfully in 2:02:00
2019-12-28T05:46:08.0044504Z == clock drift check ==
2019-12-28T05:46:08.0058257Z   local time: Sat Dec 28 05:46:08 UTC 2019
2019-12-28T05:46:08.1551779Z   network time: Sat, 28 Dec 2019 05:46:08 GMT
2019-12-28T05:46:08.1552713Z == end clock drift check ==
2019-12-28T05:46:08.8208994Z 
2019-12-28T05:46:08.8305261Z ##[error]Bash exited with code '1'.
2019-12-28T05:46:08.8357854Z ##[section]Starting: Checkout
2019-12-28T05:46:08.8359719Z ==============================================================================
2019-12-28T05:46:08.8359820Z Task         : Get sources
2019-12-28T05:46:08.8359907Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
