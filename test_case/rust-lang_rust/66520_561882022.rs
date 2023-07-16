plain
2019-12-04T23:05:46.7156932Z failures:
2019-12-04T23:05:46.7157147Z 
2019-12-04T23:05:46.7157935Z ---- [codegen] codegen/gdb_debug_script_load.rs stdout ----
2019-12-04T23:05:46.7158034Z 
2019-12-04T23:05:46.7158525Z error: verification with 'FileCheck' failed
2019-12-04T23:05:46.7158624Z status: exit code: 1
2019-12-04T23:05:46.7159165Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/gdb_debug_script_load/gdb_debug_script_load.ll" "/checkout/src/test/codegen/gdb_debug_script_load.rs"
2019-12-04T23:05:46.7159554Z ------------------------------------------
2019-12-04T23:05:46.7159617Z 
2019-12-04T23:05:46.7159828Z ------------------------------------------
2019-12-04T23:05:46.7159912Z stderr:
2019-12-04T23:05:46.7159912Z stderr:
2019-12-04T23:05:46.7160122Z ------------------------------------------
2019-12-04T23:05:46.7160229Z /checkout/src/test/codegen/gdb_debug_script_load.rs:11:11: error: CHECK: expected string not found in input
2019-12-04T23:05:46.7160598Z // CHECK: load volatile i8, i8* getelementptr inbounds ([[B:\[[0-9]* x i8\]]], [[B]]* @__rustc_debug_gdb_scripts_section__, i32 0, i32 0), align 1
2019-12-04T23:05:46.7160734Z           ^
2019-12-04T23:05:46.7161077Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/gdb_debug_script_load/gdb_debug_script_load.ll:22:17: note: scanning from here
2019-12-04T23:05:46.7161179Z define i32 @main(i32, i8**) unnamed_addr #2 {
2019-12-04T23:05:46.7161299Z 
2019-12-04T23:05:46.7161529Z ------------------------------------------
2019-12-04T23:05:46.7161574Z 
2019-12-04T23:05:46.7161605Z 
---
2019-12-04T23:05:46.7162436Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-04T23:05:46.7162544Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-04T23:05:46.7162612Z 
2019-12-04T23:05:46.7162662Z 
2019-12-04T23:05:46.7168620Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-wasm32-unknown-emscripten" "--mode" "codegen" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-04T23:05:46.7169496Z 
2019-12-04T23:05:46.7169549Z 
2019-12-04T23:05:46.7181003Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten --exclude src/libcore --exclude src/liballoc --exclude src/libproc_macro --exclude src/libstd --exclude src/libterm --exclude src/libtest
2019-12-04T23:05:46.7181187Z Build completed unsuccessfully in 2:09:13
2019-12-04T23:05:46.7181187Z Build completed unsuccessfully in 2:09:13
2019-12-04T23:05:46.7234796Z == clock drift check ==
2019-12-04T23:05:46.7247774Z   local time: Wed Dec  4 23:05:46 UTC 2019
2019-12-04T23:05:46.9987044Z   network time: Wed, 04 Dec 2019 23:05:46 GMT
2019-12-04T23:05:46.9990903Z == end clock drift check ==
2019-12-04T23:05:49.7024682Z 
2019-12-04T23:05:49.7117817Z ##[error]Bash exited with code '1'.
2019-12-04T23:05:49.7158425Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2019-12-04T23:05:49.7160267Z ==============================================================================
2019-12-04T23:05:49.7160349Z Task         : Get sources
2019-12-04T23:05:49.7160439Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
