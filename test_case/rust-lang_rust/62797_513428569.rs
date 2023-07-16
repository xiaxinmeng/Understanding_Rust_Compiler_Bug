plain
2019-07-20T02:37:15.4921233Z ---- [run-pass] run-pass/command-uid-gid.rs stdout ----
2019-07-20T02:37:15.4921280Z 
2019-07-20T02:37:15.4921353Z error: test run failed!
2019-07-20T02:37:15.4921438Z status: exit code: 101
2019-07-20T02:37:15.4921776Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/command-uid-gid/a.js"
2019-07-20T02:37:15.4922083Z ------------------------------------------
2019-07-20T02:37:15.4922123Z 
2019-07-20T02:37:15.4922331Z ------------------------------------------
2019-07-20T02:37:15.4922388Z stderr:
2019-07-20T02:37:15.4922388Z stderr:
2019-07-20T02:37:15.4922598Z ------------------------------------------
2019-07-20T02:37:15.4923123Z thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/libcore/result.rs:1051:5
2019-07-20T02:37:15.4923313Z 
2019-07-20T02:37:15.4923567Z ------------------------------------------
2019-07-20T02:37:15.4923607Z 
2019-07-20T02:37:15.4923644Z 
---
2019-07-20T02:37:15.4928200Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-20T02:37:15.4928345Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-20T02:37:15.4931831Z 
2019-07-20T02:37:15.4931923Z 
2019-07-20T02:37:15.4933669Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "run-pass" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/8.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-20T02:37:15.4934413Z 
2019-07-20T02:37:15.4934461Z 
2019-07-20T02:37:15.4939859Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/run-pass src/test/run-fail src/libstd src/liballoc src/libcore
2019-07-20T02:37:15.4939991Z Build completed unsuccessfully in 1:45:01
2019-07-20T02:37:15.4939991Z Build completed unsuccessfully in 1:45:01
2019-07-20T02:37:16.1228809Z ##[error]Bash exited with code '1'.
2019-07-20T02:37:16.1263242Z ##[section]Starting: Upload CPU usage statistics
2019-07-20T02:37:16.1270783Z ==============================================================================
2019-07-20T02:37:16.1271043Z Task         : Bash
2019-07-20T02:37:16.1271121Z Description  : Run a Bash script on macOS, Linux, or Windows
