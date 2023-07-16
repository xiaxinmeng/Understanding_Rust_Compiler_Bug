plain
2019-08-06T00:27:05.5048473Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-06T00:27:05.5048524Z 
2019-08-06T00:27:05.5048773Z   git checkout -b <new-branch-name>
2019-08-06T00:27:05.5048818Z 
2019-08-06T00:27:05.5049110Z HEAD is now at b80e8f180 Auto merge of #63311 - Centril:rollup-kt4gijw, r=Centril
2019-08-06T00:27:05.5205059Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-06T00:27:05.5208064Z ==============================================================================
2019-08-06T00:27:05.5208175Z Task         : Bash
2019-08-06T00:27:05.5208265Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-06T01:46:11.7959037Z test [ui] ui/borrowck/borrowck-vec-pattern-tail-element-loan.rs ... ok
2019-08-06T01:46:11.8424139Z test [ui] ui/borrowck/borrowck-while-break.rs ... ok
2019-08-06T01:46:11.8824533Z test [ui] ui/borrowck/borrowck-while-cond.rs ... ok
2019-08-06T01:46:11.9314226Z test [ui] ui/borrowck/borrowck-while.rs ... ok
2019-08-06T01:46:11.9681910Z test [ui] ui/borrowck/disallow-possibly-uninitialized.rs ... ok
2019-08-06T01:46:12.0541533Z test [ui] ui/borrowck/index-mut-help-with-impl.rs ... ok
2019-08-06T01:46:12.1130161Z test [ui] ui/borrowck/index-mut-help.rs ... ok
2019-08-06T01:46:12.1466994Z test [ui] ui/borrowck/borrowck-use-mut-borrow-rpass.rs ... ok
2019-08-06T01:46:12.2012223Z test [ui] ui/borrowck/issue-27282-mutation-in-guard.rs ... ok
---
2019-08-06T02:32:00.2148131Z 
2019-08-06T02:32:00.2148553Z ---- [ui] ui/issues/issue-37433.rs stdout ----
2019-08-06T02:32:00.2148635Z diff of stderr:
2019-08-06T02:32:00.2148668Z 
2019-08-06T02:32:00.2148897Z - error[E0669]: invalid value for constraint in inline assembly
2019-08-06T02:32:00.2149154Z -   --> $DIR/issue-37433.rs:5:24
2019-08-06T02:32:00.2149217Z + error[E0472]: asm! is unsupported on this target
2019-08-06T02:32:00.2149450Z +   --> $DIR/issue-37433.rs:5:9
2019-08-06T02:32:00.2149505Z 3    |
2019-08-06T02:32:00.2149565Z 4 LL |         asm!("" :: "r"(""));
2019-08-06T02:32:00.2178535Z +    |         ^^^^^^^^^^^^^^^^^^^^
2019-08-06T02:32:00.2178791Z 6 
2019-08-06T02:32:00.2179015Z 7 error: aborting due to previous error
2019-08-06T02:32:00.2179078Z 8 
2019-08-06T02:32:00.2179078Z 8 
2019-08-06T02:32:00.2179285Z 
2019-08-06T02:32:00.2179312Z 
2019-08-06T02:32:00.2179545Z The actual stderr differed from the expected stderr.
2019-08-06T02:32:00.2179845Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433/issue-37433.stderr
2019-08-06T02:32:00.2180095Z To update references, rerun the tests and pass the `--bless` flag
2019-08-06T02:32:00.2180344Z To only update this specific test, also pass `--test-args issues/issue-37433.rs`
2019-08-06T02:32:00.2180442Z error: 1 errors occurred comparing output.
2019-08-06T02:32:00.2180504Z status: exit code: 1
2019-08-06T02:32:00.2180504Z status: exit code: 1
2019-08-06T02:32:00.2181380Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37433.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433/auxiliary" "-A" "unused"
2019-08-06T02:32:00.2181919Z ------------------------------------------
2019-08-06T02:32:00.2181959Z 
2019-08-06T02:32:00.2182177Z ------------------------------------------
2019-08-06T02:32:00.2182231Z stderr:
2019-08-06T02:32:00.2182231Z stderr:
2019-08-06T02:32:00.2182432Z ------------------------------------------
2019-08-06T02:32:00.2182493Z error[E0472]: asm! is unsupported on this target
2019-08-06T02:32:00.2183072Z   --> /checkout/src/test/ui/issues/issue-37433.rs:5:9
2019-08-06T02:32:00.2183321Z    |
2019-08-06T02:32:00.2183394Z LL |         asm!("" :: "r"(""));
2019-08-06T02:32:00.2183524Z 
2019-08-06T02:32:00.2183584Z error: aborting due to previous error
2019-08-06T02:32:00.2183633Z 
2019-08-06T02:32:00.2183676Z 
---
2019-08-06T02:32:00.2188637Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-06T02:32:00.2188715Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-06T02:32:00.2207650Z 
2019-08-06T02:32:00.2207907Z 
2019-08-06T02:32:00.2209965Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "ui" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/8.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-06T02:32:00.2210700Z 
2019-08-06T02:32:00.2210734Z 
2019-08-06T02:32:00.2216041Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/ui src/test/run-fail src/libstd src/liballoc src/libcore
2019-08-06T02:32:00.2216684Z Build completed unsuccessfully in 1:59:41
2019-08-06T02:32:00.2216684Z Build completed unsuccessfully in 1:59:41
2019-08-06T02:32:01.0504323Z ##[error]Bash exited with code '1'.
2019-08-06T02:32:01.0547822Z ##[section]Starting: Upload CPU usage statistics
2019-08-06T02:32:01.0557478Z ==============================================================================
2019-08-06T02:32:01.0557591Z Task         : Bash
2019-08-06T02:32:01.0557662Z Description  : Run a Bash script on macOS, Linux, or Windows
