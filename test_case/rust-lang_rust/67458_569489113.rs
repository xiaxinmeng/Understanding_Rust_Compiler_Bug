plain
2019-12-29T09:31:28.8102806Z ---- [ui] ui/non-ice-error-on-worker-io-fail.rs stdout ----
2019-12-29T09:31:28.8102958Z 
2019-12-29T09:31:28.8103085Z error: ui test compiled successfully!
2019-12-29T09:31:28.8103246Z status: exit code: 0
2019-12-29T09:31:28.8105218Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/non-ice-error-on-worker-io-fail.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-ice-error-on-worker-io-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-o" "/dev/null" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-ice-error-on-worker-io-fail/auxiliary" "-A" "unused"
2019-12-29T09:31:28.8106233Z ------------------------------------------
2019-12-29T09:31:28.8106346Z 
2019-12-29T09:31:28.8107043Z ------------------------------------------
2019-12-29T09:31:28.8107208Z stderr:
2019-12-29T09:31:28.8107208Z stderr:
2019-12-29T09:31:28.8107650Z ------------------------------------------
2019-12-29T09:31:28.8108130Z warning: ignoring --out-dir flag due to -o flag
2019-12-29T09:31:28.8108312Z 
2019-12-29T09:31:28.8108754Z ------------------------------------------
2019-12-29T09:31:28.8108842Z 
2019-12-29T09:31:28.8108907Z 
---
2019-12-29T09:31:28.8140485Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-29T09:31:28.8140619Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-29T09:31:28.8163475Z 
2019-12-29T09:31:28.8163591Z 
2019-12-29T09:31:28.8165774Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-emscripten" "--mode" "ui" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-29T09:31:28.8166599Z 
2019-12-29T09:31:28.8166637Z 
2019-12-29T09:31:28.8169635Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten --exclude src/libcore --exclude src/liballoc --exclude src/libproc_macro --exclude src/libstd --exclude src/libterm --exclude src/libtest
2019-12-29T09:31:28.8169786Z Build completed unsuccessfully in 1:48:34
2019-12-29T09:31:28.8169786Z Build completed unsuccessfully in 1:48:34
2019-12-29T09:31:28.8245150Z == clock drift check ==
2019-12-29T09:31:28.8255638Z   local time: Sun Dec 29 09:31:28 UTC 2019
2019-12-29T09:31:29.1188125Z   network time: Sun, 29 Dec 2019 09:31:29 GMT
2019-12-29T09:31:29.1189056Z == end clock drift check ==
2019-12-29T09:31:30.0395448Z 
2019-12-29T09:31:30.0454342Z ##[error]Bash exited with code '1'.
2019-12-29T09:31:30.0491695Z ##[section]Starting: Checkout
2019-12-29T09:31:30.0493581Z ==============================================================================
2019-12-29T09:31:30.0493667Z Task         : Get sources
2019-12-29T09:31:30.0493761Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
