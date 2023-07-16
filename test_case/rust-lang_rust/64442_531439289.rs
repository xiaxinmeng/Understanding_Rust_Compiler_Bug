plain
2019-09-14T02:16:48.3334628Z failures:
2019-09-14T02:16:48.3338093Z 
2019-09-14T02:16:48.3342636Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2019-09-14T02:16:48.3342911Z 
2019-09-14T02:16:48.3343328Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-09-14T02:16:48.3343554Z status: exit code: 1
2019-09-14T02:16:48.3345541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2019-09-14T02:16:48.3347181Z ------------------------------------------
2019-09-14T02:16:48.3347409Z 
2019-09-14T02:16:48.3347807Z ------------------------------------------
2019-09-14T02:16:48.3347865Z stderr:
2019-09-14T02:16:48.3347865Z stderr:
2019-09-14T02:16:48.3348071Z ------------------------------------------
2019-09-14T02:16:48.3348137Z error[E0570]: The ABI `"stdcall"` is not supported for the current target
2019-09-14T02:16:48.3348494Z    |
2019-09-14T02:16:48.3348494Z    |
2019-09-14T02:16:48.3348547Z LL | pub extern "stdcall" fn make_stdcall() {}
2019-09-14T02:16:48.3348664Z 
2019-09-14T02:16:48.3348711Z error: aborting due to previous error
2019-09-14T02:16:48.3348760Z 
2019-09-14T02:16:48.3348984Z For more information about this error, try `rustc --explain E0570`.
---
2019-09-14T02:16:48.3366019Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-14T02:16:48.3366178Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-14T02:16:48.3371201Z 
2019-09-14T02:16:48.3371282Z 
2019-09-14T02:16:48.3378455Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "incremental" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-14T02:16:48.3379054Z 
2019-09-14T02:16:48.3379086Z 
2019-09-14T02:16:48.3392350Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
2019-09-14T02:16:48.3392453Z Build completed unsuccessfully in 2:25:21
2019-09-14T02:16:48.3392453Z Build completed unsuccessfully in 2:25:21
2019-09-14T02:16:48.3475519Z == clock drift check ==
2019-09-14T02:16:48.3490759Z   local time: Sat Sep 14 02:16:48 UTC 2019
2019-09-14T02:16:48.4468159Z   network time: Sat, 14 Sep 2019 02:16:48 GMT
2019-09-14T02:16:48.4470912Z == end clock drift check ==
2019-09-14T02:16:50.8018859Z ##[error]Bash exited with code '1'.
2019-09-14T02:16:50.8053326Z ##[section]Starting: Upload CPU usage statistics
2019-09-14T02:16:50.8061847Z ==============================================================================
2019-09-14T02:16:50.8061945Z Task         : Bash
2019-09-14T02:16:50.8062005Z Description  : Run a Bash script on macOS, Linux, or Windows
