plain
2020-02-09T15:23:29.9497676Z 4 LL | const CONSTANT: usize = limit();
2020-02-09T15:23:29.9497723Z 
2020-02-09T15:23:29.9497759Z 
2020-02-09T15:23:29.9498144Z The actual stderr differed from the expected stderr.
2020-02-09T15:23:29.9498607Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/const_eval_limit_reached.stderr
2020-02-09T15:23:29.9498966Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T15:23:29.9499559Z To only update this specific test, also pass `--test-args consts/const_limit/const_eval_limit_reached.rs`
2020-02-09T15:23:29.9499721Z error: 1 errors occurred comparing output.
2020-02-09T15:23:29.9499796Z status: exit code: 0
2020-02-09T15:23:29.9499796Z status: exit code: 0
2020-02-09T15:23:29.9501190Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_limit/const_eval_limit_reached.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/auxiliary" "-A" "unused"
2020-02-09T15:23:29.9501954Z ------------------------------------------
2020-02-09T15:23:29.9502023Z 
2020-02-09T15:23:29.9502280Z ------------------------------------------
2020-02-09T15:23:29.9502372Z stderr:
---
2020-02-09T15:23:29.9536504Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-09T15:23:29.9536649Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-09T15:23:29.9555149Z 
2020-02-09T15:23:29.9555233Z 
2020-02-09T15:23:29.9557556Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-09T15:23:29.9558368Z 
2020-02-09T15:23:29.9558406Z 
2020-02-09T15:23:29.9562230Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-09T15:23:29.9562330Z Build completed unsuccessfully in 1:37:38
2020-02-09T15:23:29.9562330Z Build completed unsuccessfully in 1:37:38
2020-02-09T15:23:29.9660853Z == clock drift check ==
2020-02-09T15:23:29.9675349Z   local time: Sun Feb  9 15:23:29 UTC 2020
2020-02-09T15:23:30.2356318Z   network time: Sun, 09 Feb 2020 15:23:30 GMT
2020-02-09T15:23:30.2356488Z == end clock drift check ==
2020-02-09T15:23:30.6889444Z 
2020-02-09T15:23:30.6984875Z ##[error]Bash exited with code '1'.
2020-02-09T15:23:30.7026809Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-09T15:23:30.7028827Z ==============================================================================
2020-02-09T15:23:30.7028937Z Task         : Get sources
2020-02-09T15:23:30.7029023Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
