plain
2020-01-01T04:34:55.2973978Z ---- [ui] ui/non-ice-error-on-worker-io-fail.rs stdout ----
2020-01-01T04:34:55.2974041Z 
2020-01-01T04:34:55.2974123Z error: ui test compiled successfully!
2020-01-01T04:34:55.2974409Z status: exit code: 0
2020-01-01T04:34:55.2976083Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/non-ice-error-on-worker-io-fail.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-ice-error-on-worker-io-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-o" "/dev/null" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-ice-error-on-worker-io-fail/auxiliary" "-A" "unused"
2020-01-01T04:34:55.2976687Z ------------------------------------------
2020-01-01T04:34:55.2976739Z 
2020-01-01T04:34:55.2976985Z ------------------------------------------
2020-01-01T04:34:55.2977079Z stderr:
2020-01-01T04:34:55.2977079Z stderr:
2020-01-01T04:34:55.2977329Z ------------------------------------------
2020-01-01T04:34:55.2977591Z warning: ignoring --out-dir flag due to -o flag
2020-01-01T04:34:55.2977816Z 
2020-01-01T04:34:55.2978082Z ------------------------------------------
2020-01-01T04:34:55.2978128Z 
2020-01-01T04:34:55.2978175Z 
---
2020-01-01T04:34:55.3027010Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-01T04:34:55.3027691Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-01T04:34:55.3052105Z 
2020-01-01T04:34:55.3052194Z 
2020-01-01T04:34:55.3058186Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "ui" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-01T04:34:55.3059304Z 
2020-01-01T04:34:55.3059386Z 
2020-01-01T04:34:55.3064316Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
2020-01-01T04:34:55.3064460Z Build completed unsuccessfully in 2:21:58
2020-01-01T04:34:55.3064460Z Build completed unsuccessfully in 2:21:58
2020-01-01T04:34:55.3224654Z == clock drift check ==
2020-01-01T04:34:55.3241183Z   local time: Wed Jan  1 04:34:55 UTC 2020
2020-01-01T04:34:55.8589871Z   network time: Wed, 01 Jan 2020 04:34:55 GMT
2020-01-01T04:34:55.8591007Z == end clock drift check ==
2020-01-01T04:34:56.6852296Z 
2020-01-01T04:34:56.6961182Z ##[error]Bash exited with code '1'.
2020-01-01T04:34:56.7015938Z ##[section]Starting: Checkout
2020-01-01T04:34:56.7018919Z ==============================================================================
2020-01-01T04:34:56.7019024Z Task         : Get sources
2020-01-01T04:34:56.7019103Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
